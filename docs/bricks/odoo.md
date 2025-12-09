# Odoo Brick

The Odoo brick integrates with Odoo ERP to perform operations on invoices, products, and other business objects.

## Configuration

```json
{
  "brick_type": "odoo",
  "config": {
    "api_url": "https://your-odoo-instance.com",
    "database": "your-database",
    "username": "your-username",
    "api_key": "your-api-key",
    "operation": "create_invoice",
    "model": "account.move",
    "fields": {
      "partner_id": "{{partner_id}}",
      "invoice_line_ids": "{{line_items}}"
    }
  }
}
```

## Configuration Options

- **api_url** (required): Your Odoo instance URL
- **database** (required): Odoo database name
- **username** (required): Odoo username
- **api_key** (required): API key or password
- **operation** (required): Operation type (`create`, `read`, `update`, `delete`)
- **model** (required): Odoo model name (e.g., `account.move`, `product.product`)
- **fields** (optional): Fields to set or filter

## Supported Operations

### Create Invoice

```json
{
  "brick_type": "odoo",
  "config": {
    "api_url": "https://odoo.example.com",
    "database": "mydb",
    "username": "admin",
    "api_key": "password",
    "operation": "create",
    "model": "account.move",
    "fields": {
      "partner_id": "{{customer_id}}",
      "invoice_date": "{{invoice_date}}",
      "invoice_line_ids": "{{line_items}}"
    }
  }
}
```

### Get Products

```json
{
  "brick_type": "odoo",
  "config": {
    "api_url": "https://odoo.example.com",
    "database": "mydb",
    "username": "admin",
    "api_key": "password",
    "operation": "read",
    "model": "product.product",
    "fields": {
      "ids": "{{product_ids}}"
    }
  }
}
```

## Input Format

Varies by operation. For create operations:

```json
{
  "customer_id": 123,
  "invoice_date": "2025-01-01",
  "line_items": [
    {
      "product_id": 456,
      "quantity": 2,
      "price_unit": 100.0
    }
  ]
}
```

## Output Format

Returns Odoo API response format:

```json
{
  "id": 789,
  "name": "INV/2025/0001",
  "state": "draft"
}
```

## Use Cases

- Automatically create invoices from orders
- Sync product information
- Update customer records
- Generate reports

## Common Models

- **account.move**: Invoices
- **product.product**: Products
- **res.partner**: Customers/Suppliers
- **sale.order**: Sales Orders
- **purchase.order**: Purchase Orders

