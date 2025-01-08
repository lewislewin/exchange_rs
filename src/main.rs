use warp::Filter;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: Uuid,
    order_type: OrderType,
    ticker: String,
    price: f64,
    quantity: u32,
}

#[derive(Debug)]
struct OrderBook {
    buy_orders: VecDeque<Order>,
    sell_orders: VecDeque<Order>,
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            buy_orders: VecDeque::new(),
            sell_orders: VecDeque::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        match order.order_type {
            OrderType::Buy => self.buy_orders.push_back(order.clone()),
            OrderType::Sell => self.sell_orders.push_back(order.clone()),
        }
        info!("Order added: {:?}", order);
        self.match_orders();
    }

    fn match_orders(&mut self) {
        while let (Some(buy_order), Some(sell_order)) = (self.buy_orders.front(), self.sell_orders.front()) {
            if buy_order.price >= sell_order.price {
                let trade_quantity = buy_order.quantity.min(sell_order.quantity);

                info!(
                    "Trade executed: {} shares of {} at ${:.2}",
                    trade_quantity, buy_order.ticker, sell_order.price
                );

                if buy_order.quantity > sell_order.quantity {
                    self.buy_orders[0].quantity -= trade_quantity;
                    self.sell_orders.pop_front();
                } else if sell_order.quantity > buy_order.quantity {
                    self.sell_orders[0].quantity -= trade_quantity;
                    self.buy_orders.pop_front();
                } else {
                    self.buy_orders.pop_front();
                    self.sell_orders.pop_front();
                }
            } else {
                break;
            }
        }
    }
}

#[derive(Debug)]
struct StockExchange {
    order_books: HashMap<String, Arc<Mutex<OrderBook>>>,
}

impl StockExchange {
    fn new() -> Self {
        StockExchange {
            order_books: HashMap::new(),
        }
    }

    fn place_order(&mut self, order: Order) {
        // Get or create the order book for the ticker
        let book = self
            .order_books
            .entry(order.ticker.clone())
            .or_insert_with(|| Arc::new(Mutex::new(OrderBook::new())))
            .clone();

        // Lock the order book and process the order
        let mut book = book.lock().unwrap();
        book.add_order(order);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting server...");

    let exchange = Arc::new(Mutex::new(StockExchange::new()));

    let place_order = {
        let exchange = exchange.clone();
        warp::post()
            .and(warp::path("place_order"))
            .and(warp::body::json())
            .map(move |order: Order| {
                let mut exchange = exchange.lock().unwrap();
                exchange.place_order(order.clone());
                info!("Order processed: {:?}", order);
                warp::reply::json(&order)
            })
    };

    let routes = place_order;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
