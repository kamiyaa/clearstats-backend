use std::time::Duration;

use axum::http::{HeaderMap, HeaderValue};
use hex::ToHex;
use hmac::{Hmac, Mac};
use reqwest::{Method, StatusCode};
use sha2::Sha256;

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    integrations::stripe::client::STRIPE_API_VERSION,
    utils::request::DEFAULT_TIMEOUT,
};

pub fn create_authenticated_request(
    secret_key: &str,
    method: Method,
    url: &str,
) -> reqwest::RequestBuilder {
    let bearer = format!("Bearer {secret_key}");
    let mut headers = HeaderMap::new();
    headers.append(
        "Authorization",
        HeaderValue::from_str(&bearer).expect("Failed to serialize bearer"),
    );
    headers.append(
        "Stripe-Version",
        HeaderValue::from_str(&STRIPE_API_VERSION).expect("Failed to serialize bearer"),
    );
    reqwest::Client::new()
        .request(method, url)
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
        .headers(headers)
}

const STRIPE_SIGNATURE: &str = "stripe-signature";

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug)]
pub struct StripeSignature {
    pub t: u64,
    pub v1: String,
    pub v0: Option<String>,
}

impl StripeSignature {
    pub fn from_header_map(header_map: &HeaderMap) -> AppServerResult<Self> {
        let header_value = header_map.get(STRIPE_SIGNATURE).ok_or_else(|| {
            let error_msg = "Missing Stripe-Signature";
            tracing::debug!("{error_msg}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string())
        })?;

        let signature = header_value.to_str().map_err(|err| {
            let error_msg = "Failed to parse Stripe-Signature";
            tracing::debug!("{error_msg}: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;

        let split_values: Vec<_> = signature.split(',').collect();

        let mut t: Option<u64> = None;
        let mut v1: Option<String> = None;
        let mut v0: Option<String> = None;

        for value in split_values {
            if value.starts_with("t=") {
                let v = value.split_at("t=".len()).1.parse().map_err(|err| {
                    let error_msg = "Failed to parse t value";
                    tracing::debug!("{error_msg}: {err}");
                    ServerErrorResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        1234,
                        error_msg.to_string(),
                    )
                })?;
                t = Some(v);
            } else if value.starts_with("v1=") {
                let v = value.split_at("v1=".len()).1.to_string();
                v1 = Some(v);
            } else if value.starts_with("v0=") {
                let v = value.split_at("v0=".len()).1.to_string();
                v0 = Some(v);
            }
        }

        match (t, v1) {
            (Some(t), Some(v1)) => Ok(StripeSignature { t, v1, v0 }),
            _ => {
                let error_msg = "Failed to extract all Stripe signature values";
                tracing::debug!(signature, "{error_msg}");
                let err = ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1234,
                    error_msg.to_string(),
                );
                Err(err)
            }
        }
    }

    pub fn verify(&self, secret: &str, payload: &str) -> AppServerResult {
        let signed_payload = format!("{}.{payload}", self.t);

        let mut hmac = HmacSha256::new_from_slice(secret.as_bytes()).map_err(|err| {
            let error_msg = "Failed to create HMAC";
            tracing::debug!("{error_msg}: {err}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;
        hmac.update(signed_payload.as_bytes());

        let result = hmac.finalize().into_bytes();

        let computed_signature: String = result.to_vec().encode_hex();
        if computed_signature != self.v1 {
            let error_msg = "Invalid signature";
            tracing::debug!(
                computed = computed_signature,
                given = self.v1,
                "{error_msg}"
            );
            let err = ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            );
            return Err(err);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_001_validate_stripe_signature() {
        let mut headers = HeaderMap::new();

        const WEBHOOK_SIGNING_SECRET: &str =
            "whsec_750e601a79a762e75cf1ec30b41b2e6f88daafcc644b3a9f29e735b4453ebb9a";
        const SIGNATURE: &str = "t=1746757390,v1=bc889a3926c751e3ea753efc027f6adbb11d224b57ebae9e86c0da62461e9f35,v0=537d1b7f8ebf0452111cafdcd6ee4cf80eb1b1ba62bc2ded7569289edfa0a2b0";
        const PAYLOAD: &str = r#"{
  "id": "evt_1RMgagCMHTTliF5iVoysIuMJ",
  "object": "event",
  "api_version": "2025-03-31.basil",
  "created": 1746757389,
  "data": {
    "object": {
      "id": "in_1RMgafCMHTTliF5ilpBkRLZf",
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
      "created": 1746757387,
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
      "effective_at": 1746757387,
      "ending_balance": 0,
      "footer": null,
      "from_invoice": null,
      "hosted_invoice_url": "https://invoice.stripe.com/i/acct_1RCUo0CMHTTliF5i/test_YWNjdF8xUkNVbzBDTUhUVGxpRjVpLF9TSEY0YlNFQXc2dzlFcFNCekFOcjduNVV6Z0JtQVVSLDEzNzI5ODE5MA0200HbVagulC?s=ap",
      "invoice_pdf": "https://pay.stripe.com/invoice/acct_1RCUo0CMHTTliF5i/test_YWNjdF8xUkNVbzBDTUhUVGxpRjVpLF9TSEY0YlNFQXc2dzlFcFNCekFOcjduNVV6Z0JtQVVSLDEzNzI5ODE5MA0200HbVagulC/pdf?s=ap",
      "issuer": {
        "type": "self"
      },
      "last_finalization_error": null,
      "latest_revision": null,
      "lines": {
        "object": "list",
        "data": [
          {
            "id": "il_1RMgafCMHTTliF5iQYZRgR73",
            "object": "line_item",
            "amount": 4999,
            "currency": "cad",
            "description": "1 × Basic (at $49.99 / month)",
            "discount_amounts": [

            ],
            "discountable": true,
            "discounts": [

            ],
            "invoice": "in_1RMgafCMHTTliF5ilpBkRLZf",
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
                "subscription": "sub_1RMgafCMHTTliF5imBKfvfyV",
                "subscription_item": "si_SHF4BPhLdrKPYz"
              },
              "type": "subscription_item_details"
            },
            "period": {
              "end": 1749435787,
              "start": 1746757387
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
        "url": "/v1/invoices/in_1RMgafCMHTTliF5ilpBkRLZf/lines"
      },
      "livemode": false,
      "metadata": {
      },
      "next_payment_attempt": null,
      "number": "KP9VDVTM-0019",
      "on_behalf_of": null,
      "parent": {
        "quote_details": null,
        "subscription_details": {
          "metadata": {
          },
          "subscription": "sub_1RMgafCMHTTliF5imBKfvfyV"
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
      "period_end": 1746757387,
      "period_start": 1746757387,
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
        "finalized_at": 1746757387,
        "marked_uncollectible_at": null,
        "paid_at": 1746757388,
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
    "idempotency_key": "ea765962-90ba-4b1e-bc4c-a2db3130cdad"
  },
  "type": "invoice.payment_succeeded"
}"#;

        headers.insert(STRIPE_SIGNATURE, SIGNATURE.parse().unwrap());

        let signature =
            StripeSignature::from_header_map(&headers).expect("Failed to parse stripe signature");

        signature
            .verify(WEBHOOK_SIGNING_SECRET, PAYLOAD)
            .expect("Signature mismatch");
    }
}
