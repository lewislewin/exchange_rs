# Stock Exchange HTTP Server

A simple HTTP-based stock exchange application built with Rust and Warp. This application simulates an order-matching engine where buy and sell orders are placed and matched based on price and quantity.

## Features

- **Order Matching**: Automatically matches buy and sell orders based on price and quantity.
- **Concurrency Support**: Thread-safe handling of orders and order books.
- **HTTP API**: Provides a RESTful API for interacting with the stock exchange.
- **Real-Time Logging**: Logs every order addition, processing, and trade execution.

## Endpoints

### Place an Order
**POST** `/place_order`

Place a new buy or sell order.

#### Request Body
```json
{
    "id": "string (UUID)",
    "order_type": "Buy | Sell",
    "ticker": "string (e.g., 'AAPL')",
    "price": "float",
    "quantity": "integer"
}
