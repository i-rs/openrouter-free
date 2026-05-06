use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/models";

#[derive(Parser, Debug)]
#[command(
    name = "openrouter-free",
    about = "List free models available on OpenRouter",
    version = "0.1.0"
)]
struct Args {
    #[arg(short, long, default_value = "10")]
    limit: usize,

    #[arg(short, long, help = "Show all models including non-free ones")]
    all: bool,

    #[arg(short, long, help = "Show verbose output with all details")]
    verbose: bool,

    #[arg(short = 'j', long, help = "Output in JSON format")]
    json: bool,

    #[arg(
        short = 's',
        long,
        value_enum,
        default_value = "context",
        help = "Sort by field"
    )]
    sort: SortBy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum SortBy {
    Context,
    Name,
    Id,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Context => write!(f, "context"),
            SortBy::Name => write!(f, "name"),
            SortBy::Id => write!(f, "id"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ModelsResponse {
    data: Vec<Model>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Model {
    id: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    pricing: Option<Pricing>,
    #[serde(default, alias = "context_length")]
    context_length: Option<u32>,
    #[serde(default)]
    architecture: Option<Architecture>,
    #[serde(default)]
    supported_parameters: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Pricing {
    #[serde(default, alias = "prompt")]
    prompt: Option<String>,
    #[serde(default, alias = "completion")]
    completion: Option<String>,
    #[serde(default, alias = "request")]
    request: Option<String>,
    #[serde(default, alias = "image")]
    image: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Architecture {
    #[serde(default)]
    modality: Option<String>,
    #[serde(default)]
    input_modalities: Vec<String>,
    #[serde(default)]
    output_modalities: Vec<String>,
    #[serde(default)]
    tokenizer: Option<String>,
    #[serde(default)]
    instruct_type: Option<String>,
}

fn is_free_model(model: &Model) -> bool {
    model.id.ends_with(":free")
}

fn format_price(price_str: &Option<String>) -> String {
    match price_str {
        Some(p) if p == "0" => "Free".to_string(),
        Some(p) => format!("${}", p),
        None => "N/A".to_string(),
    }
}

fn print_model_table(models: &[Model], verbose: bool) {
    println!("\n{}", "=".repeat(120));
    println!(
        "{: <8} {: <50} {: <15} {: <12} DESCRIPTION",
        "IDX", "MODEL ID", "CONTEXT", "PROMPT"
    );
    println!("{}", "=".repeat(120));

    for (idx, model) in models.iter().enumerate() {
        let context = model
            .context_length
            .map(|c| {
                if c >= 1_000_000 {
                    format!("{}M", c / 1_000_000)
                } else if c >= 1000 {
                    format!("{}K", c / 1000)
                } else {
                    c.to_string()
                }
            })
            .unwrap_or_else(|| "N/A".to_string());

        let prompt_price = format_price(&model.pricing.as_ref().and_then(|p| p.prompt.clone()));
        let desc = if verbose {
            model.description.clone()
        } else {
            model
                .description
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(50)
                .collect::<String>()
        };

        println!(
            "{: <8} {: <50} {: <15} {: <12} {}",
            idx + 1,
            &model.id[..model.id.len().min(48)],
            context,
            prompt_price,
            desc
        );
    }
    println!("{}\n", "=".repeat(120));
}

fn print_json(models: &[Model]) {
    let output: Vec<serde_json::Value> = models
        .iter()
        .map(|m| {
            serde_json::json!({
                "id": m.id,
                "name": m.name,
                "description": m.description,
                "context_length": m.context_length,
                "prompt_price": m.pricing.as_ref().and_then(|p| p.prompt.clone()),
                "completion_price": m.pricing.as_ref().and_then(|p| p.completion.clone()),
                "modality": m.architecture.as_ref().and_then(|a| a.modality.clone()).unwrap_or_default(),
            })
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    println!("Fetching models from OpenRouter...");

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Connecting to OpenRouter API...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let response = client
        .get(OPENROUTER_API_URL)
        .header("User-Agent", "openrouter-free-cli/0.1.0")
        .send()
        .await?;

    pb.finish_with_message("Models received!");

    if !response.status().is_success() {
        eprintln!("Error: API returned status {}", response.status());
        std::process::exit(1);
    }

    let models_response: ModelsResponse = response.json().await?;

    let filtered_models: Vec<Model> = if args.all {
        models_response.data
    } else {
        models_response
            .data
            .into_iter()
            .filter(is_free_model)
            .collect()
    };

    let total_count = filtered_models.len();

    let mut sorted_models = filtered_models;
    match args.sort {
        SortBy::Context => {
            sorted_models.sort_by(|a, b| {
                let a_ctx = a.context_length.unwrap_or(0);
                let b_ctx = b.context_length.unwrap_or(0);
                b_ctx.cmp(&a_ctx)
            });
        }
        SortBy::Name => {
            sorted_models.sort_by(|a, b| {
                let a_name = a.name.as_deref().unwrap_or("");
                let b_name = b.name.as_deref().unwrap_or("");
                a_name.to_lowercase().cmp(&b_name.to_lowercase())
            });
        }
        SortBy::Id => {
            sorted_models.sort_by_key(|a| a.id.to_lowercase());
        }
    }

    let display_models: Vec<Model> = if args.limit > 0 {
        sorted_models.into_iter().take(args.limit).collect()
    } else {
        sorted_models
    };

    if args.json {
        print_json(&display_models);
    } else {
        let total = if args.all {
            total_count
        } else {
            display_models.len()
        };

        let model_type = if args.all { "total" } else { "free" };
        println!(
            "\nFound {} {} models (sorted by {}, showing top {}):",
            total,
            model_type,
            args.sort,
            display_models.len().min(args.limit)
        );

        print_model_table(&display_models, args.verbose);

        if !args.verbose {
            println!("Tip: Use -v flag to see full descriptions");
        }
    }

    Ok(())
}
