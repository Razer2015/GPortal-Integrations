# G-Portal Integrations

## Work In Progress

## Configurations

### Environment variables

| Variable name            | Required | Default value            | Description                                                                                                                |
| ------------------------ | -------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| CHRONO_TIMEZONE          | No       | Europe/Helsinki          | Possible values: https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html                                                   |
| ------------------------ | -------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| GPORTAL_USERNAME         | Yes      |                          |                                                                                                                            |
| GPORTAL_PASSWORD         | Yes      |                          |                                                                                                                            |
| TOTP_SECRET              | No       |                          | TOTP Secret is required if you have 2 Factor Authentication enabled on your G-Portal account.                              |
| ------------------------ | -------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| DISCORD_DONATION_WEBHOOK | No       |                          | Webhook URL you can create from Discord channel integrations page. If not given, donations are not polled.                 |
| DONATION_INTERVAL        | No       | 900_000 (15 minutes)     | Interval in which the donations are polled.                                                                                |
| VIP_MANAGEMENT_URL       | No       |                          | Url added in the donation embed for quickly accessing the VIP management site.                                             |
| ------------------------ | -------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| RUST_LOG                 | No       | info                     | Log level used for logging (`error`, `warn`, `info`, `debug`, `trace`).                                                    |
| ------------------------ | -------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
|||||

### Notes
