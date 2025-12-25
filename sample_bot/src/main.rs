use dotenv::dotenv;
use kalshi::Kalshi;
use std::env;

extern crate kalshi;

fn retrieve_credentials() -> Result<(String, String), String> {
    let key_id = env::var("KALSHI_DEMO_API_KEY")
        .map_err(|_| "KALSHI_DEMO_API_KEY environment variable not set")?;
    let pem_path = env::var("KALSHI_DEMO_PEM_PATH")
        .map_err(|_| "KALSHI_DEMO_PEM_PATH environment variable not set")?;
    Ok((key_id, pem_path))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (key_id, pem_path) = match retrieve_credentials() {
        Ok(creds) => creds,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Please set KALSHI_DEMO_API_KEY and KALSHI_DEMO_PEM_PATH environment variables");
            return;
        }
    };

    // Create authenticated Kalshi instance
    let kalshi_instance = match Kalshi::new(kalshi::TradingEnvironment::DemoMode, &key_id, &pem_path).await {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Failed to authenticate: {:?}", e);
            return;
        }
    };

    // Example: Get a market
    let new_york_ticker = "HIGHNY-23NOV13-T51".to_string();

    let _nytemp_market_data = match kalshi_instance.get_market(&new_york_ticker).await {
        Ok(market) => {
            println!("Market data: {:?}", market);
            market
        }
        Err(e) => {
            eprintln!("Failed to get market data: {:?}", e);
            return;
        }
    };

    let _nytemp_market_orderbook = match kalshi_instance
        .get_orderbook(&new_york_ticker, Some(10))
        .await
    {
        Ok(orderbook) => {
            println!("Orderbook: {:?}", orderbook);
            orderbook
        }
        Err(e) => {
            eprintln!("Failed to get orderbook: {:?}", e);
            return;
        }
    };

    // Example: Create and cancel an order (commented out to prevent accidental trading)
    // let bought_order = kalshi_instance
    //     .create_order(
    //         kalshi::Action::Buy,
    //         None,
    //         1,
    //         kalshi::Side::Yes,
    //         new_york_ticker,
    //         kalshi::OrderType::Limit,
    //         None,  // buy_max_cost
    //         None,  // expiration_ts
    //         Some(5), // yes_price (5 cents)
    //         None,  // no_price
    //         None,  // sell_position_floor
    //         None,  // yes_price_dollars
    //         None,  // no_price_dollars
    //         None,  // time_in_force
    //         None,  // post_only
    //         None,  // reduce_only
    //         None,  // self_trade_prevention_type
    //         None,  // order_group_id
    //         None,  // cancel_order_on_pause
    //     )
    //     .await
    //     .unwrap();
    //
    // let ny_order_id = bought_order.order_id.clone();
    // let cancelled_order = kalshi_instance.cancel_order(&ny_order_id).await.unwrap();
    // println!("{:?}", cancelled_order);

    println!("Sample bot completed successfully!");
}
