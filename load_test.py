import requests
import concurrent.futures
import uuid
import random

# Server URL
BASE_URL = "http://127.0.0.1:3030/place_order"

# Generate matching order data
def generate_matching_orders():
    ticker = random.choice(["AAPL", "GOOG", "TSLA", "MSFT"])
    sell_price = round(random.uniform(100, 150), 2)
    buy_price = round(random.uniform(sell_price, 200), 2)

    sell_order = {
        "id": str(uuid.uuid4()),
        "order_type": "Sell",
        "ticker": ticker,
        "price": sell_price,
        "quantity": random.randint(10, 100)
    }

    buy_order = {
        "id": str(uuid.uuid4()),
        "order_type": "Buy",
        "ticker": ticker,
        "price": buy_price,
        "quantity": random.randint(10, 100)
    }

    return sell_order, buy_order

# Send a single order request
def send_order(order):
    try:
        response = requests.post(BASE_URL, json=order)
        if response.status_code == 200:
            print(f"Success: {response.json()}")
        else:
            print(f"Failed: {response.status_code} - {response.text}")
    except Exception as e:
        print(f"Error: {e}")

# Load test with concurrent requests
def load_test(concurrent_requests=100, iterations=10):
    with concurrent.futures.ThreadPoolExecutor(max_workers=concurrent_requests) as executor:
        for _ in range(iterations):
            orders = []
            for _ in range(concurrent_requests // 2):
                sell_order, buy_order = generate_matching_orders()
                orders.append(sell_order)
                orders.append(buy_order)
            futures = [executor.submit(send_order, order) for order in orders]
            concurrent.futures.wait(futures)

if __name__ == "__main__":
    load_test(concurrent_requests=50, iterations=20)
