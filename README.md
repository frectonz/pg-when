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

<details>

<summary>Example 2: Future Weekday with AM/PM</summary>

Finds the time for next Friday at 8:00 PM in the `America/New_York` timezone.

```sql
SELECT when_is('next friday at 8:00 pm in America/New_York');
SELECT seconds_at('next friday at 8:00 pm in America/New_York');
SELECT millis_at('next friday at 8:00 pm in America/New_York');
SELECT micros_at('next friday at 8:00 pm in America/New_York');
```

</details>

<details>

<summary>Example 3: Future Relative Date with a UTC Offset</summary>

Calculates the timestamp for exactly 2 months from now, at midnight, in the `UTC-8` timezone.

```sql
SELECT when_is('in 2 months at midnight in UTC-8');
SELECT seconds_at('in 2 months at midnight in UTC-8');
SELECT millis_at('in 2 months at midnight in UTC-8');
SELECT micros_at('in 2 months at midnight in UTC-8');
```

</details>

<details>

<summary>Example 4: Specific Time During the Previous Week</summary>

Gets the timestamp for last Monday at a specific 24-hour time: 22:30 (10:30 PM).

```sql
SELECT when_is('last monday at 22:30');
SELECT seconds_at('last monday at 22:30');
SELECT millis_at('last monday at 22:30');
SELECT micros_at('last monday at 22:30');
```

</details>

<details>

<summary>Example 5: Specific Date with a Time Keyword</summary>

Gets the timestamp for the evening (6 PM) on a specific date, New Year's Eve 2026. This shows how an exact date can be combined with a relative time keyword.

```sql
SELECT when_is('December 31, 2026 at evening');
SELECT seconds_at('December 31, 2026 at evening');
SELECT millis_at('December 31, 2026 at evening');
SELECT micros_at('December 31, 2026 at evening');
```

</details>

## Query Combinations

A `pg-when` query has up to three components: a date, a time, and a timezone. These components can be combined in several ways, connected by the keywords `at` and `in`.

All of the following are valid.

```sql
-- all
SELECT when_is('<date> at <time> in <timezone>');

-- date only
SELECT when_is('<date>');
SELECT when_is('<date> in <timezone>');

-- time only
SELECT when_is('<time>');
SELECT when_is('<time> in <timezone>');

-- date and time
SELECT when_is('<date> at <time>');
```

_NOTE: If a timezone is not provided, the system defaults to UTC._
