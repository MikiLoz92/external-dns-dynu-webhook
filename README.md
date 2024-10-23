# `external-dns` Dynu webhook provider

The following application is a webhook provider implementation that updates DNS records on Dynu. You may use this to keep in sync the resources in your k8s cluster with the DNS zone of your Dynu account.

# How to

When installing `external-dns` as a Helm chart, be sure to provide the following arguments in the chart's `values.yaml`:
```yaml
provider:
  name: webhook
  webhook:
    image:
      repository: mikiloz/external-dns-dynu-webhook
      tag: latest # or whichever other version you want to use, check the GH tags
    env:
      - name: DYNU_API_KEY
        value: # here goes your API key
      - name: DYNU_DOMAIN_NAMES
        value: mydomain.com,myotherdomain.com # comma-separated list of domains (no trailing commas!)
```
