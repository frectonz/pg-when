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

A `pg-when` query has up to three components: a **date**, a **time**, and a **timezone**. These components can be combined in several ways, connected by the keywords `at` and `in`.

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

## Component Details

### `<date>`

A date can be either **relative** or **exact**.

#### Relative Date

Describes a date in relation to the present.

- **Keywords**: `today`, `yesterday`, `tomorrow`
- **Phrases**: `next week`, `last month`, `this friday`, `5 days ago`, `in 2 years`

#### Exact Date

Specifies a calendar date.

- `YYYY-MM-DD`, `YYYY/MM/DD` (e.g. `2004-10-10`, `2004/10/10`)
- `DD-MM-YYYY`, `DD/MM/YYYY` (e.g. `10-10-2004`, `10/10/2004`)
- `Month D, YYYY` (e.g. `January 10, 2004`)
- `D Month YYYY` (e.g. `10 Jan 2004`)

### `<time>`

A time can be either **relative** or **exact**.

#### Relative Time

Describes a general or relative time.

- **Keywords**: `noon` (12 PM), `midnight` (12 AM), `morning` (9 AM), `evening` (6 PM)
- **Phrases**: `next hour`, `previous minute`, `this hour`

#### Exact Date

Specifies aw precise time on the clock.

- **AM/PM Format**: `H:MM:SS AM/PM`, `H:MM:SS am/pm` (e.g. `8:30 pm`, `8:30 PM`)
- **GMT/24-Hour Format**: `HH:MM:SS GMT`, `HH:MM:SS` (e.g. `15:45 GMT`, `15:45`)

### `<timezone>`

A timezone can be a **named reference** or a **UTC offset**.

#### Named Timezone

Uses standard IANA timezone names.

- **Examples**: `America/New_York`, `Europe/London`, `Africa/Addis_Ababa`

#### UTC Offset

Specifies the hours and minutes offset from Coordinated Universal Time.

- **Examples**: `UTC+3`, `UTC-08:00`, `UTC+05:30`

## Available Functions

The following functions are provided by the Postgres extension.

| Function                  | Return Type                | Description                                          |
| ------------------------- | -------------------------- | ---------------------------------------------------- |
| `when_is`     | `TIMESTAMP WITH TIME ZONE` | Returns a standard PostgreSQL timestamp.             |
| `seconds_at`  | `BIGINT`                   | Returns the total seconds since the UNIX epoch.      |
| `millis_at`   | `BIGINT`                   | Returns the total milliseconds since the UNIX epoch. |
| `micros_at`   | `BIGINT`                   | Returns the total microseconds since the UNIX epoch. |

## Usage

Run a PostgreSQL database that has `pg-when` already installed.

```bash
docker run --network=host frectonz/pg-when
```

`pg-when` is also distributed with other PostgreSQL versions.

### PostgreSQL 13

```bash
docker run --network=host frectonz/pg-when:pg13
```

### PostgreSQL 14

```bash
docker run --network=host frectonz/pg-when:pg14
```

### PostgreSQL 15

```bash
docker run --network=host frectonz/pg-when:pg15
```

### PostgreSQL 16

```bash
docker run --network=host frectonz/pg-when:pg16
```

### PostgreSQL 17

```bash
docker run --network=host frectonz/pg-when:pg17
```

### PostgreSQL 18

```bash
docker run --network=host frectonz/pg-when:pg18
```
