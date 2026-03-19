use serde::{Deserialize, Serialize};

use super::StripeObject;

// API Version: 2025-03-31.basil
// https://docs.stripe.com/api/events/types

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Event {
    pub id: String,
    pub object: String,
    pub api_version: String,
    pub created: u64,
    pub data: EventDataObject,
    pub livemode: bool,
    pub request: EventRequest,
    #[serde(rename = "type")]
    pub event_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EventDataObject {
    pub object: StripeObject,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EventRequest {
    pub id: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentMethodOption {}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_001_deserialize_event_checkout_session_complete() {
        const EVENT: &str = r#"{
            "id": "evt_1RMcdSCMHTTliF5issIUBut6",
            "object": "event",
            "api_version": "2025-03-31.basil",
            "created": 1746742186,
            "data": {
              "object": {
                "id": "cs_test_a1KWGRHKoc4XzUIjaYYF3Ml1EAgzp2QfFIhl2WBn1hQxH3ZCdCA61Ld5H8",
                "object": "checkout.session",
                "adaptive_pricing": null,
                "after_expiration": null,
                "allow_promotion_codes": null,
                "amount_subtotal": 4999,
                "amount_total": 4999,
                "automatic_tax": {
                  "enabled": false,
                  "liability": null,
                  "provider": null,
                  "status": null
                },
                "billing_address_collection": null,
                "cancel_url": null,
                "client_reference_id": null,
                "client_secret": null,
                "collected_information": {
                  "shipping_details": null
                },
                "consent": null,
                "consent_collection": null,
                "created": 1746742166,
                "currency": "cad",
                "currency_conversion": null,
                "custom_fields": [

                ],
                "custom_text": {
                  "after_submit": null,
                  "shipping_address": null,
                  "submit": null,
                  "terms_of_service_acceptance": null
                },
                "customer": "cus_SGqVI5XZS5sDjA",
                "customer_creation": null,
                "customer_details": {
                  "address": {
                    "city": null,
                    "country": "CA",
                    "line1": null,
                    "line2": null,
                    "postal_code": "A1A 1A1",
                    "state": null
                  },
                  "email": "jeff@indaggo.com",
                  "name": "alice-lab",
                  "phone": null,
                  "tax_exempt": "none",
                  "tax_ids": [

                  ]
                },
                "customer_email": null,
                "discounts": [

                ],
                "expires_at": 1746828566,
                "invoice": "in_1RMcdRCMHTTliF5i3unGNWXV",
                "invoice_creation": null,
                "livemode": false,
                "locale": null,
                "metadata": {
                },
                "mode": "subscription",
                "payment_intent": null,
                "payment_link": null,
                "payment_method_collection": "always",
                "payment_method_configuration_details": {
                  "id": "pmc_1RCUoWCMHTTliF5iI4cdKCqM",
                  "parent": null
                },
                "payment_method_options": {
                  "card": {
                    "request_three_d_secure": "automatic"
                  }
                },
                "payment_method_types": [
                  "card",
                  "link"
                ],
                "payment_status": "paid",
                "permissions": null,
                "phone_number_collection": {
                  "enabled": false
                },
                "recovered_from": null,
                "saved_payment_method_options": {
                  "allow_redisplay_filters": [
                    "always"
                  ],
                  "payment_method_remove": null,
                  "payment_method_save": null
                },
                "setup_intent": null,
                "shipping_address_collection": null,
                "shipping_cost": null,
                "shipping_options": [

                ],
                "status": "complete",
                "submit_type": null,
                "subscription": "sub_1RMcdRCMHTTliF5iM812gr0j",
                "success_url": "http://localhost:5010/labspaces/alice-lab/billing",
                "total_details": {
                  "amount_discount": 0,
                  "amount_shipping": 0,
                  "amount_tax": 0
                },
                "ui_mode": "hosted",
                "url": null,
                "wallet_options": null
              }
            },
            "livemode": false,
            "pending_webhooks": 2,
            "request": {
              "id": null,
              "idempotency_key": null
            },
            "type": "checkout.session.completed"
          }
        "#;

        let obj: Event =
            serde_json::from_str(EVENT).expect("Failed to deserialize stripe event JSON");
        assert_eq!(obj.event_type, "checkout.session.completed");
    }

    #[test]
    fn test_002_deserialize_event_checkout_session_complete() {
        const EVENT: &str = r#"{
    "id": "evt_1RMcdTCMHTTliF5iW5D0gg39",
    "object": "event",
    "api_version": "2025-03-31.basil",
    "created": 1746742186,
    "data": {
      "object": {
        "id": "in_1RMcdRCMHTTliF5i3unGNWXV",
        "object": "invoice",
        "account_country": "CA",
        "account_name": "Indaggo sandbox",
        "account_tax_ids": null,
        "amount_due": 4999,
        "amount_overpaid": 0,
        "amount_paid": 4999,
        "amount_remaining": 0,
        "amount_shipping": 0,
        "application": null,
        "attempt_count": 0,
        "attempted": true,
        "auto_advance": false,
        "automatic_tax": {
          "disabled_reason": null,
          "enabled": false,
          "liability": null,
          "provider": null,
          "status": null
        },
        "automatically_finalizes_at": null,
        "billing_reason": "subscription_create",
        "collection_method": "charge_automatically",
        "created": 1746742184,
        "currency": "cad",
        "custom_fields": null,
        "customer": "cus_SGqVI5XZS5sDjA",
        "customer_address": null,
        "customer_email": "jeff@indaggo.com",
        "customer_name": "alice-lab",
        "customer_phone": null,
        "customer_shipping": null,
        "customer_tax_exempt": "none",
        "customer_tax_ids": [

        ],
        "default_payment_method": null,
        "default_source": null,
        "default_tax_rates": [

        ],
        "description": null,
        "discounts": [

        ],
        "due_date": null,
        "effective_at": 1746742184,
        "ending_balance": 0,
        "footer": null,
        "from_invoice": null,
        "hosted_invoice_url": "https://invoice.stripe.com/i/acct_1RCUo0CMHTTliF5i/test_YWNjdF8xUkNVbzBDTUhUVGxpRjVpLF9TSEF6WXA0SmNyYjlGY29OT2VnOWptRjUxZWxOY1BGLDEzNzI4Mjk4Nw0200FOSfjNOY?s=ap",
        "invoice_pdf": "https://pay.stripe.com/invoice/acct_1RCUo0CMHTTliF5i/test_YWNjdF8xUkNVbzBDTUhUVGxpRjVpLF9TSEF6WXA0SmNyYjlGY29OT2VnOWptRjUxZWxOY1BGLDEzNzI4Mjk4Nw0200FOSfjNOY/pdf?s=ap",
        "issuer": {
          "type": "self"
        },
        "last_finalization_error": null,
        "latest_revision": null,
        "lines": {
          "object": "list",
          "data": [
            {
              "id": "il_1RMcdRCMHTTliF5iogrL09ny",
              "object": "line_item",
              "amount": 4999,
              "currency": "cad",
              "description": "1 × Basic (at $49.99 / month)",
              "discount_amounts": [

              ],
              "discountable": true,
              "discounts": [

              ],
              "invoice": "in_1RMcdRCMHTTliF5i3unGNWXV",
              "livemode": false,
              "metadata": {
              },
              "parent": {
                "invoice_item_details": null,
                "subscription_item_details": {
                  "invoice_item": null,
                  "proration": false,
                  "proration_details": {
                    "credited_items": null
                  },
                  "subscription": "sub_1RMcdRCMHTTliF5iM812gr0j",
                  "subscription_item": "si_SHAzusqzALaQ7S"
                },
                "type": "subscription_item_details"
              },
              "period": {
                "end": 1749420584,
                "start": 1746742184
              },
              "pretax_credit_amounts": [

              ],
              "pricing": {
                "price_details": {
                  "price": "price_1RCV6tCMHTTliF5iuMgrQbyE",
                  "product": "prod_S6iXedXCQhbo7o"
                },
                "type": "price_details",
                "unit_amount_decimal": "4999"
              },
              "quantity": 1,
              "taxes": [

              ]
            }
          ],
          "has_more": false,
          "total_count": 1,
          "url": "/v1/invoices/in_1RMcdRCMHTTliF5i3unGNWXV/lines"
        },
        "livemode": false,
        "metadata": {
        },
        "next_payment_attempt": null,
        "number": "KP9VDVTM-0003",
        "on_behalf_of": null,
        "parent": {
          "quote_details": null,
          "subscription_details": {
            "metadata": {
            },
            "subscription": "sub_1RMcdRCMHTTliF5iM812gr0j"
          },
          "type": "subscription_details"
        },
        "payment_settings": {
          "default_mandate": null,
          "payment_method_options": {
            "acss_debit": null,
            "bancontact": null,
            "card": {
              "request_three_d_secure": "automatic"
            },
            "customer_balance": null,
            "konbini": null,
            "sepa_debit": null,
            "us_bank_account": null
          },
          "payment_method_types": null
        },
        "period_end": 1746742184,
        "period_start": 1746742184,
        "post_payment_credit_notes_amount": 0,
        "pre_payment_credit_notes_amount": 0,
        "receipt_number": null,
        "rendering": null,
        "shipping_cost": null,
        "shipping_details": null,
        "starting_balance": 0,
        "statement_descriptor": null,
        "status": "paid",
        "status_transitions": {
          "finalized_at": 1746742184,
          "marked_uncollectible_at": null,
          "paid_at": 1746742184,
          "voided_at": null
        },
        "subtotal": 4999,
        "subtotal_excluding_tax": 4999,
        "test_clock": null,
        "total": 4999,
        "total_discount_amounts": [

        ],
        "total_excluding_tax": 4999,
        "total_pretax_credit_amounts": [

        ],
        "total_taxes": [

        ],
        "webhooks_delivered_at": null
      }
    },
    "livemode": false,
    "pending_webhooks": 2,
    "request": {
      "id": null,
      "idempotency_key": "08d5a0b5-e8d7-459c-94c8-c43f7d6815dc"
    },
    "type": "invoice.finalized"
  }"#;

        let obj: Event =
            serde_json::from_str(EVENT).expect("Failed to deserialize stripe event JSON");
        assert_eq!(obj.event_type, "invoice.finalized");
    }
}
