# anyOf

The `anyOf` generator generates one or multiple random value(s) from a list of possible values.
This generator has two inputs:

- `values`: An array of possible values to generate from. These values may be any generator.
- `num`: The number of values to generate. Defaults to `1`. If set to `0`, all values will be generated.
  If set to a negative number, a random number of values will be generated.

## Examples

Generate a random value from a list of possible values:

```json
{
  "type": "anyOf",
  "values": [
    {
      "type": "string",
      "value": "test"
    },
    {
      "type": "string",
      "value": "test2"
    }
  ]
}
```

Generate a random number of values from a list of possible values:

```json
{
  "type": "anyOf",
  "num": -1,
  "values": [
    {
      "type": "string",
      "value": "test"
    },
    {
      "type": "string",
      "value": "test2"
    }
  ]
}
```