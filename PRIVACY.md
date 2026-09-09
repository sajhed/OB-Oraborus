# Privacy

OB Oraborus is local-first and contains no hidden analytics, advertising, or data-sale logic.

## Default state

- Local Ollama preferred
- Cloud providers disabled until configured
- Microphone off
- Camera off
- Screen capture off
- No approved indexing folders
- Persistent memory on but write requires explicit user action and approval policy
- Financial execution off
- Message auto-send off

## Local Only

Local Only blocks cloud AI and external web operations, prefers loopback providers, keeps memory and indexed files local, and disables external telemetry. The interface displays **LOCAL ONLY** while active.

## Context acquisition

OB may use only context permitted by the corresponding switch: microphone, camera, screen, filesystem, browser, network, AI provider, and memory. Turning a switch off stops new acquisition; existing approved records remain visible for deletion or export.

## Memory control

Users can inspect, edit, delete, export, import, forget one item, or forget all approved memories. Credential and payment-like patterns are blocked. Embeddings are data derived from the approved text and follow the same deletion policy.

## Screen and audio retention

Screen images and voice recordings are not retained by default. A workflow that needs an artifact must show its destination and retention policy. General logs store only state and redacted metadata.

## External services

When a cloud provider or connector is enabled, the requested data is sent to that provider under its terms. OB shows the selected provider and model in diagnostics. Connection errors never fall back to an external provider when Local Only is active.

## Deletion and backup

Application data resides in the Windows application-data directory. Backups may include settings, conversations, memories, automations, agent configuration, and API customizations. Raw secrets are excluded because they remain in Windows Credential Manager. Deleting application data does not automatically delete credential-vault entries; use the in-app provider disconnect action.
