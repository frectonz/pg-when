# pg-when

<img src="./logo.jpeg" alt="pg-when-logo" width="256" align="right" style="margin: 0 0 1em 1em;"/>

A PostgreSQL extension for creating time values with natural language.

<details>

<summary>Example 1: Relative Past in a Specific Timezone</summary>

Gets the timestamp for the beginning of the current hour, but 5 days ago, in the `Asia/Tokyo` timezone.

```sql
SELECT when_is('5 days ago at this hour in Asia/Tokyo');
SELECT seconds_at('5 days ago at this hour in Asia/Tokyo');
SELECT millis_at('5 days ago at this hour in Asia/Tokyo');
SELECT micros_at('5 days ago at this hour in Asia/Tokyo');
```

</details>
