# Monobank API Client

This is a synchronous Monobank API wrapper written entirely in Rust. It has been implemented solely for learning purposes. For more details on the Monobank API, refer to their [official documentation](https://api.monobank.ua/docs/index.html).

## How to Use

### Requests

There are 3 available requests:

1. `request_currencies()`
2. `request_user_info()`
3. `request_payments(account, from, to)`

### Integration Example

Once you receive the data from these requests, you can integrate it into your workflow. For example, I use it to automatically update a Google Spreadsheet where I track my finances.

### Setup

To get started, follow these steps:

1. **Create a Token:** Obtain your API token from [Monobank's API portal](https://api.monobank.ua/index.html).
   
2. **Set Environment Variable:** Export your API token in your local environment: `export API_KEY=<your token here>`
