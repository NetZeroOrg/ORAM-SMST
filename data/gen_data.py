import csv
import random
import string

def generate_random_email():
    """
    Generates a random email address.
    """
    domains = ["gmail.com", "yahoo.com", "outlook.com", "example.com"]
    name = ''.join(random.choices(string.ascii_lowercase + string.digits, k=10))
    return f"{name}@{random.choice(domains)}"


def generate_asset_data(num_users: int, num_assets: int, output_file: str = "data.csv"):
    """
    Generates a CSV file with random user emails and asset values.

    :param num_users: Number of user rows to generate
    :param num_assets: Number of assets per user
    :param output_file: Name of the CSV file to write data
    """
    # Define asset names dynamically
    asset_names = [f"Asset_{i + 1}" for i in range(num_assets)]

    with open(output_file, mode="w", newline="") as file:
        writer = csv.writer(file)

        # Write the header row
        writer.writerow(["UserEmail"] + asset_names)

        for _ in range(num_users):
            user_email = generate_random_email()
            asset_values = [round(random.randint(1, 10000), 2) for _ in range(num_assets)]
            writer.writerow([user_email] + asset_values)

    print(f"Data generation complete. File saved as '{output_file}'.")


if __name__ == "__main__":
    # User inputs for number of users and assets
    num_users = 100
    num_assets = 3
    output_file = "data.csv"

    generate_asset_data(num_users, num_assets, output_file)
