## Table of Contents

  - [Page Not Found404Page Not FoundWe couldn't find the page. Maybe you were looking for one of these pages below?FIX API OverviewAPI ChangelogGet API KeysPage Not Found](#page-not-found404page-not-foundwe-couldnt-find-the-page-maybe-you-were-looking-for-one-of-these-pages-belowfix-api-overviewapi-changelogget-api-keyspage-not-found)
    - [Get Exchange Status - API DocumentationGet Exchange Status](#get-exchange-status-api-documentationget-exchange-status)
      - [Get Exchange Announcements - API DocumentationGet Exchange Announcements](#get-exchange-announcements-api-documentationget-exchange-announcements)
        - [Get Series Fee Changes - API DocumentationGet Series Fee Changes](#get-series-fee-changes-api-documentationget-series-fee-changes)
          - [Get Exchange Schedule - API DocumentationGet Exchange Schedule](#get-exchange-schedule-api-documentationget-exchange-schedule)
            - [Create API Key - API DocumentationCreate API Key](#create-api-key-api-documentationcreate-api-key)
            - [Delete API Key - API DocumentationDelete API Key](#delete-api-key-api-documentationdelete-api-key)
            - [Generate API Key - API DocumentationGenerate API Key](#generate-api-key-api-documentationgenerate-api-key)
            - [Get API Keys - API DocumentationGet API Keys](#get-api-keys-api-documentationget-api-keys)
            - [Accept Quote - API DocumentationAccept Quote](#accept-quote-api-documentationaccept-quote)
            - [Confirm Quote - API DocumentationConfirm Quote](#confirm-quote-api-documentationconfirm-quote)
            - [Create Quote - API DocumentationCreate Quote](#create-quote-api-documentationcreate-quote)
            - [Create RFQ - API DocumentationCreate RFQ](#create-rfq-api-documentationcreate-rfq)
            - [Delete Quote - API DocumentationDelete Quote](#delete-quote-api-documentationdelete-quote)
            - [Delete RFQ - API DocumentationDelete RFQ](#delete-rfq-api-documentationdelete-rfq)
            - [Get Communications ID - API DocumentationGet Communications ID](#get-communications-id-api-documentationget-communications-id)
            - [Get Quote - API DocumentationGet Quote](#get-quote-api-documentationget-quote)
            - [Get Quotes - API DocumentationGet Quotes](#get-quotes-api-documentationget-quotes)
            - [Get RFQ - API DocumentationGet RFQ](#get-rfq-api-documentationget-rfq)
            - [Get RFQs - API DocumentationGet RFQs](#get-rfqs-api-documentationget-rfqs)
            - [Get Event - API DocumentationGet Event](#get-event-api-documentationget-event)
            - [Get Event Candlesticks - API DocumentationGet Event Candlesticks](#get-event-candlesticks-api-documentationget-event-candlesticks)
            - [Get Event Forecast Percentile History - API DocumentationGet Event Forecast Percentile History](#get-event-forecast-percentile-history-api-documentationget-event-forecast-percentile-history)
            - [Get Event Metadata - API DocumentationGet Event Metadata](#get-event-metadata-api-documentationget-event-metadata)
            - [Get Events - API DocumentationGet Events](#get-events-api-documentationget-events)
            - [Get Multivariate Events - API DocumentationGet Multivariate Events](#get-multivariate-events-api-documentationget-multivariate-events)
            - [Get User Data Timestamp - API DocumentationGet User Data Timestamp](#get-user-data-timestamp-api-documentationget-user-data-timestamp)
            - [Get FCM Orders - API DocumentationGet FCM Orders](#get-fcm-orders-api-documentationget-fcm-orders)
            - [Get FCM Positions - API DocumentationGet FCM Positions](#get-fcm-positions-api-documentationget-fcm-positions)
            - [Get Incentives - API DocumentationGet Incentives](#get-incentives-api-documentationget-incentives)
            - [Get Live Data - API DocumentationGet Live Data](#get-live-data-api-documentationget-live-data)
            - [Get Multiple Live Data - API DocumentationGet Multiple Live Data](#get-multiple-live-data-api-documentationget-multiple-live-data)
            - [Batch Get Market Candlesticks - API DocumentationBatch Get Market Candlesticks](#batch-get-market-candlesticks-api-documentationbatch-get-market-candlesticks)
            - [Get Market - API DocumentationGet Market](#get-market-api-documentationget-market)
            - [Get Market Candlesticks - API DocumentationGet Market Candlesticks](#get-market-candlesticks-api-documentationget-market-candlesticks)
            - [Get Market Orderbook - API DocumentationGet Market Orderbook](#get-market-orderbook-api-documentationget-market-orderbook)
            - [Get Markets - API DocumentationGet Markets](#get-markets-api-documentationget-markets)
            - [Get Series - API DocumentationGet Series](#get-series-api-documentationget-series)
            - [Get Series List - API DocumentationGet Series List](#get-series-list-api-documentationget-series-list)
            - [Get Trades - API DocumentationGet Trades](#get-trades-api-documentationget-trades)
            - [Get Milestone - API DocumentationGet Milestone](#get-milestone-api-documentationget-milestone)
            - [Get Milestones - API DocumentationGet Milestones](#get-milestones-api-documentationget-milestones)
            - [Create Market In Multivariate Event Collection - API DocumentationCreate Market In Multivariate Event Collection](#create-market-in-multivariate-event-collection-api-documentationcreate-market-in-multivariate-event-collection)
            - [Get Multivariate Event Collection - API DocumentationGet Multivariate Event Collection](#get-multivariate-event-collection-api-documentationget-multivariate-event-collection)
            - [Get Multivariate Event Collection Lookup History - API DocumentationGet Multivariate Event Collection Lookup History](#get-multivariate-event-collection-lookup-history-api-documentationget-multivariate-event-collection-lookup-history)
            - [Get Multivariate Event Collections - API DocumentationGet Multivariate Event Collections](#get-multivariate-event-collections-api-documentationget-multivariate-event-collections)
            - [Lookup Tickers For Market In Multivariate Event Collection - API DocumentationLookup Tickers For Market In Multivariate Event Collection](#lookup-tickers-for-market-in-multivariate-event-collection-api-documentationlookup-tickers-for-market-in-multivariate-event-collection)
            - [Create Order Group - API DocumentationCreate Order Group](#create-order-group-api-documentationcreate-order-group)
            - [Delete Order Group - API DocumentationDelete Order Group](#delete-order-group-api-documentationdelete-order-group)
            - [Get Order Group - API DocumentationGet Order Group](#get-order-group-api-documentationget-order-group)
            - [Get Order Groups - API DocumentationGet Order Groups](#get-order-groups-api-documentationget-order-groups)
            - [Reset Order Group - API DocumentationReset Order Group](#reset-order-group-api-documentationreset-order-group)
            - [Amend Order - API DocumentationAmend Order](#amend-order-api-documentationamend-order)
            - [Batch Cancel Orders - API DocumentationBatch Cancel Orders](#batch-cancel-orders-api-documentationbatch-cancel-orders)
            - [Batch Create Orders - API DocumentationBatch Create Orders](#batch-create-orders-api-documentationbatch-create-orders)
            - [Cancel Order - API DocumentationCancel Order](#cancel-order-api-documentationcancel-order)
            - [Create Order - API DocumentationCreate Order](#create-order-api-documentationcreate-order)
            - [Decrease Order - API DocumentationDecrease Order](#decrease-order-api-documentationdecrease-order)
            - [Get Order - API DocumentationGet Order](#get-order-api-documentationget-order)
            - [Get Order Queue Position - API DocumentationGet Order Queue Position](#get-order-queue-position-api-documentationget-order-queue-position)
            - [Get Orders - API DocumentationGet Orders](#get-orders-api-documentationget-orders)
            - [Get Queue Positions for Orders - API DocumentationGet Queue Positions for Orders](#get-queue-positions-for-orders-api-documentationget-queue-positions-for-orders)
            - [Get Balance - API DocumentationGet Balance](#get-balance-api-documentationget-balance)
            - [Get Fills - API DocumentationGet Fills](#get-fills-api-documentationget-fills)
            - [Get Positions - API DocumentationGet Positions](#get-positions-api-documentationget-positions)
            - [Get Settlements - API DocumentationGet Settlements](#get-settlements-api-documentationget-settlements)
            - [Get Total Resting Order Value - API DocumentationGet Total Resting Order Value](#get-total-resting-order-value-api-documentationget-total-resting-order-value)
            - [Get Filters for Sports - API DocumentationGet Filters for Sports](#get-filters-for-sports-api-documentationget-filters-for-sports)
            - [Get Tags for Series Categories - API DocumentationGet Tags for Series Categories](#get-tags-for-series-categories-api-documentationget-tags-for-series-categories)
            - [Get Structured Target - API DocumentationGet Structured Target](#get-structured-target-api-documentationget-structured-target)
            - [Get Structured Targets - API DocumentationGet Structured Targets](#get-structured-targets-api-documentationget-structured-targets)

---

## Page Not Found404Page Not FoundWe couldn't find the page. Maybe you were looking for one of these pages below?FIX API OverviewAPI ChangelogGet API KeysPage Not Found

404

# Page Not Found

We couldn't find the page. Maybe you were looking for one of these pages below?

[FIX API Overview](/fix/index#) [API Changelog](/changelog/index#api-changelog) [Get API Keys](/api-reference/api-keys/get-api-keys#)


---

### Get Exchange Status - API DocumentationGet Exchange Status

#### Response

200

application/json

Exchange status retrieved successfully

[​](#response-exchange-active)

exchange\_active

boolean

required

False if the core Kalshi exchange is no longer taking any state changes at all. This includes but is not limited to trading, new users, and transfers. True unless we are under maintenance.

[​](#response-trading-active)

trading\_active

boolean

required

True if we are currently permitting trading on the exchange. This is true during trading hours and false outside exchange hours. Kalshi reserves the right to pause at any time in case issues are detected.

[​](#response-exchange-estimated-resume-time-one-of-0)

exchange\_estimated\_resume\_time

string<date-time> \| null

Estimated downtime for the current exchange maintenance window. However, this is not guaranteed and can be extended.


---

#### Get Exchange Announcements - API DocumentationGet Exchange Announcements

#### Response

200

application/json

Exchange announcements retrieved successfully

[​](#response-announcements)

announcements

object\[\]

required

A list of exchange-wide announcements.

Showchild attributes


---

##### Get Series Fee Changes - API DocumentationGet Series Fee Changes

#### Query Parameters

[​](#parameter-series-ticker)

series\_ticker

string

[​](#parameter-show-historical)

show\_historical

boolean

default:false

#### Response

200

application/json

Series fee changes retrieved successfully

[​](#response-series-fee-change-arr)

series\_fee\_change\_arr

object\[\]

required

Showchild attributes


---

###### Get Exchange Schedule - API DocumentationGet Exchange Schedule

#### Response

200

application/json

Exchange schedule retrieved successfully

[​](#response-schedule)

schedule

object

required

Showchild attributes


---

###### Create API Key - API DocumentationCreate API Key

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-name)

name

string

required

Name for the API key. This helps identify the key's purpose

[​](#body-public-key)

public\_key

string

required

RSA public key in PEM format. This will be used to verify signatures on API requests

[​](#body-scopes)

scopes

string\[\]

List of scopes to grant to the API key. Valid values are 'read' and 'write'. If 'write' is included, 'read' must also be included. Defaults to full access (\['read', 'write'\]) if not provided.

#### Response

201

application/json

API key created successfully

[​](#response-api-key-id)

api\_key\_id

string

required

Unique identifier for the newly created API key


---

###### Delete API Key - API DocumentationDelete API Key

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-api-key)

api\_key

string

required

API key ID to delete

#### Response

204

API key successfully deleted


---

###### Generate API Key - API DocumentationGenerate API Key

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-name)

name

string

required

Name for the API key. This helps identify the key's purpose

[​](#body-scopes)

scopes

string\[\]

List of scopes to grant to the API key. Valid values are 'read' and 'write'. If 'write' is included, 'read' must also be included. Defaults to full access (\['read', 'write'\]) if not provided.

#### Response

201

application/json

API key generated successfully

[​](#response-api-key-id)

api\_key\_id

string

required

Unique identifier for the newly generated API key

[​](#response-private-key)

private\_key

string

required

RSA private key in PEM format. This must be stored securely and cannot be retrieved again after this response


---

###### Get API Keys - API DocumentationGet API Keys

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Response

200

application/json

List of API keys retrieved successfully

[​](#response-api-keys)

api\_keys

object\[\]

required

List of all API keys associated with the user

Showchild attributes


---

###### Accept Quote - API DocumentationAccept Quote

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-quote-id)

quote\_id

string

required

Quote ID

#### Body

application/json

[​](#body-accepted-side)

accepted\_side

enum<string>

required

The side of the quote to accept (yes or no)

Available options:

`yes`,

`no`

#### Response

204

Quote accepted successfully


---

###### Confirm Quote - API DocumentationConfirm Quote

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-quote-id)

quote\_id

string

required

Quote ID

#### Body

application/json

An empty response body

#### Response

204

Quote confirmed successfully


---

###### Create Quote - API DocumentationCreate Quote

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-rfq-id)

rfq\_id

string

required

The ID of the RFQ to quote on

[​](#body-yes-bid)

yes\_bid

string

required

The bid price for YES contracts, in dollars

Example:

`"0.5600"`

[​](#body-no-bid)

no\_bid

string

required

The bid price for NO contracts, in dollars

Example:

`"0.5600"`

[​](#body-rest-remainder)

rest\_remainder

boolean

required

Whether to rest the remainder of the quote after execution

#### Response

201

application/json

Quote created successfully

[​](#response-id)

id

string

required

The ID of the newly created quote


---

###### Create RFQ - API DocumentationCreate RFQ

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-market-ticker)

market\_ticker

string

required

The ticker of the market for which to create an RFQ

[​](#body-rest-remainder)

rest\_remainder

boolean

required

Whether to rest the remainder of the RFQ after execution

[​](#body-contracts)

contracts

integer

The number of contracts for the RFQ

[​](#body-target-cost-centi-cents)

target\_cost\_centi\_cents

integer<int64>

The target cost for the RFQ in centi-cents

[​](#body-replace-existing)

replace\_existing

boolean

default:false

Whether to delete existing RFQs as part of this RFQ's creation

[​](#body-subtrader-id)

subtrader\_id

string

The subtrader to create the RFQ for (FCM members only)

#### Response

201

application/json

RFQ created successfully

[​](#response-id)

id

string

required

The ID of the newly created RFQ


---

###### Delete Quote - API DocumentationDelete Quote

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-quote-id)

quote\_id

string

required

Quote ID

#### Response

204

Quote deleted successfully


---

###### Delete RFQ - API DocumentationDelete RFQ

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-rfq-id)

rfq\_id

string

required

RFQ ID

#### Response

204

RFQ deleted successfully


---

###### Get Communications ID - API DocumentationGet Communications ID

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Response

200

application/json

Communications ID retrieved successfully

[​](#response-communications-id)

communications\_id

string

required

A public communications ID which is used to identify the user


---

###### Get Quote - API DocumentationGet Quote

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-quote-id)

quote\_id

string

required

Quote ID

#### Response

200

application/json

Quote retrieved successfully

[​](#response-quote)

quote

object

required

The details of the requested quote

Showchild attributes


---

###### Get Quotes - API DocumentationGet Quotes

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).

[​](#parameter-market-ticker)

market\_ticker

string

Filter by market ticker

[​](#parameter-limit)

limit

integer<int32>

default:500

Parameter to specify the number of results per page. Defaults to 500.

Required range: `1 <= x <= 500`

[​](#parameter-status)

status

string

Filter quotes by status

[​](#parameter-quote-creator-user-id)

quote\_creator\_user\_id

string

Filter quotes by quote creator user ID

[​](#parameter-rfq-creator-user-id)

rfq\_creator\_user\_id

string

Filter quotes by RFQ creator user ID

[​](#parameter-rfq-creator-subtrader-id)

rfq\_creator\_subtrader\_id

string

Filter quotes by RFQ creator subtrader ID (FCM members only)

[​](#parameter-rfq-id)

rfq\_id

string

Filter quotes by RFQ ID

#### Response

200

application/json

Quotes retrieved successfully

[​](#response-quotes)

quotes

object\[\]

required

List of quotes matching the query criteria

Showchild attributes

[​](#response-cursor)

cursor

string

Cursor for pagination to get the next page of results


---

###### Get RFQ - API DocumentationGet RFQ

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-rfq-id)

rfq\_id

string

required

RFQ ID

#### Response

200

application/json

RFQ retrieved successfully

[​](#response-rfq)

rfq

object

required

The details of the requested RFQ

Showchild attributes


---

###### Get RFQs - API DocumentationGet RFQs

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).

[​](#parameter-market-ticker)

market\_ticker

string

Filter by market ticker

[​](#parameter-limit)

limit

integer<int32>

default:100

Parameter to specify the number of results per page. Defaults to 100.

Required range: `1 <= x <= 100`

[​](#parameter-status)

status

string

Filter RFQs by status

[​](#parameter-creator-user-id)

creator\_user\_id

string

Filter RFQs by creator user ID

#### Response

200

application/json

RFQs retrieved successfully

[​](#response-rfqs)

rfqs

object\[\]

required

List of RFQs matching the query criteria

Showchild attributes

[​](#response-cursor)

cursor

string

Cursor for pagination to get the next page of results


---

###### Get Event - API DocumentationGet Event

#### Path Parameters

[​](#parameter-event-ticker)

event\_ticker

string

required

Event ticker

#### Query Parameters

[​](#parameter-with-nested-markets)

with\_nested\_markets

boolean

default:false

If true, markets are included within the event object. If false (default), markets are returned as a separate top-level field in the response.

#### Response

200

application/json

Event retrieved successfully

[​](#response-event)

event

object

required

Data for the event.

Showchild attributes

[​](#response-markets)

markets

object\[\]

required

Data for the markets in this event. This field is deprecated in favour of the "markets" field inside the event. Which will be filled with the same value if you use the query parameter "with\_nested\_markets=true".

Showchild attributes


---

###### Get Event Candlesticks - API DocumentationGet Event Candlesticks

#### Path Parameters

[​](#parameter-ticker)

ticker

string

required

The event ticker

[​](#parameter-series-ticker)

series\_ticker

string

required

The series ticker

#### Query Parameters

[​](#parameter-start-ts)

start\_ts

integer<int64>

required

Start timestamp for the range

[​](#parameter-end-ts)

end\_ts

integer<int64>

required

End timestamp for the range

[​](#parameter-period-interval)

period\_interval

enum<integer>

required

Specifies the length of each candlestick period, in minutes. Must be one minute, one hour, or one day.

Available options:

`1`,

`60`,

`1440`

#### Response

200

application/json

Event candlesticks retrieved successfully

[​](#response-market-tickers)

market\_tickers

string\[\]

required

Array of market tickers in the event.

[​](#response-market-candlesticks)

market\_candlesticks

object\[\]\[\]

required

Array of market candlestick arrays, one for each market in the event.

Showchild attributes

[​](#response-adjusted-end-ts)

adjusted\_end\_ts

integer<int64>

required

Adjusted end timestamp if the requested candlesticks would be larger than maxAggregateCandidates.


---

###### Get Event Forecast Percentile History - API DocumentationGet Event Forecast Percentile History

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-ticker)

ticker

string

required

The event ticker

[​](#parameter-series-ticker)

series\_ticker

string

required

The series ticker

#### Query Parameters

[​](#parameter-percentiles)

percentiles

integer<int32>\[\]

required

Array of percentile values to retrieve (0-10000, max 10 values)

Maximum array length: `10`

Required range: `0 <= x <= 10000`

[​](#parameter-start-ts)

start\_ts

integer<int64>

required

Start timestamp for the range

[​](#parameter-end-ts)

end\_ts

integer<int64>

required

End timestamp for the range

[​](#parameter-period-interval)

period\_interval

enum<integer>

required

Specifies the length of each forecast period, in minutes. 0 for 5-second intervals, or 1, 60, or 1440 for minute-based intervals.

Available options:

`0`,

`1`,

`60`,

`1440`

#### Response

200

application/json

Event forecast percentile history retrieved successfully

[​](#response-forecast-history)

forecast\_history

object\[\]

required

Array of forecast percentile data points over time.

Showchild attributes


---

###### Get Event Metadata - API DocumentationGet Event Metadata

#### Path Parameters

[​](#parameter-event-ticker)

event\_ticker

string

required

Event ticker

#### Response

200

application/json

Event metadata retrieved successfully

[​](#response-image-url)

image\_url

string

required

A path to an image that represents this event.

[​](#response-market-details)

market\_details

object\[\]

required

Metadata for the markets in this event.

Showchild attributes

[​](#response-settlement-sources)

settlement\_sources

object\[\]

required

A list of settlement sources for this event.

Showchild attributes

[​](#response-featured-image-url)

featured\_image\_url

string

A path to an image that represents the image of the featured market.

[​](#response-competition-one-of-0)

competition

string \| null

Event competition.

[​](#response-competition-scope-one-of-0)

competition\_scope

string \| null

Event scope, based on the competition.


---

###### Get Events - API DocumentationGet Events

#### Query Parameters

[​](#parameter-limit)

limit

integer

default:200

Parameter to specify the number of results per page. Defaults to 200. Maximum value is 200.

Required range: `1 <= x <= 200`

[​](#parameter-cursor)

cursor

string

Parameter to specify the pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

[​](#parameter-with-nested-markets)

with\_nested\_markets

boolean

default:false

Parameter to specify if nested markets should be included in the response. When true, each event will include a 'markets' field containing a list of Market objects associated with that event.

[​](#parameter-with-milestones)

with\_milestones

boolean

default:false

If true, includes related milestones as a field alongside events.

[​](#parameter-status)

status

enum<string>

Filter by event status. Possible values are 'open', 'closed', 'settled'. Leave empty to return events with any status.

Available options:

`open`,

`closed`,

`settled`

[​](#parameter-series-ticker)

series\_ticker

string

Filter by series ticker

[​](#parameter-min-close-ts)

min\_close\_ts

integer<int64>

Filter events with at least one market with close timestamp greater than this Unix timestamp (in seconds).

#### Response

200

application/json

Events retrieved successfully

[​](#response-events)

events

object\[\]

required

Array of events matching the query criteria.

Showchild attributes

[​](#response-cursor)

cursor

string

required

Pagination cursor for the next page. Empty if there are no more results.

[​](#response-milestones)

milestones

object\[\]

Array of milestones related to the events.

Showchild attributes


---

###### Get Multivariate Events - API DocumentationGet Multivariate Events

#### Query Parameters

[​](#parameter-limit)

limit

integer

default:100

Number of results per page. Defaults to 100. Maximum value is 200.

Required range: `1 <= x <= 200`

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results.

[​](#parameter-series-ticker)

series\_ticker

string

Filter by series ticker

[​](#parameter-collection-ticker)

collection\_ticker

string

Filter events by collection ticker. Returns only multivariate events belonging to the specified collection. Cannot be used together with series\_ticker.

[​](#parameter-with-nested-markets)

with\_nested\_markets

boolean

default:false

Parameter to specify if nested markets should be included in the response. When true, each event will include a 'markets' field containing a list of Market objects associated with that event.

#### Response

200

application/json

Multivariate events retrieved successfully

[​](#response-events)

events

object\[\]

required

Array of multivariate events matching the query criteria.

Showchild attributes

[​](#response-cursor)

cursor

string

required

Pagination cursor for the next page. Empty if there are no more results.


---

###### Get User Data Timestamp - API DocumentationGet User Data Timestamp

#### Response

200

application/json

User data timestamp retrieved successfully

[​](#response-as-of-time)

as\_of\_time

string<date-time>

required

Timestamp when user data was last updated.


---

###### Get FCM Orders - API DocumentationGet FCM Orders

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-subtrader-id)

subtrader\_id

string

required

Restricts the response to orders for a specific subtrader (FCM members only)

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).

[​](#parameter-ticker)

ticker

string

Filter by market ticker

[​](#parameter-min-ts)

min\_ts

integer<int64>

Restricts the response to orders after a timestamp, formatted as a Unix Timestamp

[​](#parameter-max-ts)

max\_ts

integer<int64>

Restricts the response to orders before a timestamp, formatted as a Unix Timestamp

[​](#parameter-status)

status

enum<string>

Restricts the response to orders that have a certain status

Available options:

`resting`,

`canceled`,

`executed`

[​](#parameter-limit)

limit

integer

Parameter to specify the number of results per page. Defaults to 100

Required range: `1 <= x <= 1000`

#### Response

200

application/json

Orders retrieved successfully

[​](#response-orders)

orders

object\[\]

required

Showchild attributes

[​](#response-cursor)

cursor

string

required


---

###### Get FCM Positions - API DocumentationGet FCM Positions

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-subtrader-id)

subtrader\_id

string

required

Restricts the response to positions for a specific subtrader (FCM members only)

[​](#parameter-ticker)

ticker

string

Ticker of desired positions

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions

[​](#parameter-count-filter)

count\_filter

string

Restricts the positions to those with any of following fields with non-zero values, as a comma separated list

[​](#parameter-settlement-status)

settlement\_status

enum<string>

Settlement status of the markets to return. Defaults to unsettled

Available options:

`all`,

`unsettled`,

`settled`

[​](#parameter-limit)

limit

integer

Parameter to specify the number of results per page. Defaults to 100

Required range: `1 <= x <= 1000`

[​](#parameter-cursor)

cursor

string

The Cursor represents a pointer to the next page of records in the pagination

#### Response

200

application/json

Positions retrieved successfully

[​](#response-market-positions)

market\_positions

object\[\]

required

List of market positions

Showchild attributes

[​](#response-event-positions)

event\_positions

object\[\]

required

List of event positions

Showchild attributes

[​](#response-cursor)

cursor

string

The Cursor represents a pointer to the next page of records in the pagination. Use the value returned here in the cursor query parameter for this end-point to get the next page containing limit records. An empty value of this field indicates there is no next page.


---

###### Get Incentives - API DocumentationGet Incentives

#### Query Parameters

[​](#parameter-status)

status

enum<string>

Status filter. Can be "all", "active", "upcoming", "closed", or "paid\_out". Default is "all".

Available options:

`all`,

`active`,

`upcoming`,

`closed`,

`paid_out`

[​](#parameter-type)

type

enum<string>

Type filter. Can be "all", "liquidity", or "volume". Default is "all".

Available options:

`all`,

`liquidity`,

`volume`

[​](#parameter-limit)

limit

integer

Number of results per page. Defaults to 100. Maximum value is 10000.

Required range: `1 <= x <= 10000`

[​](#parameter-cursor)

cursor

string

Cursor for pagination

#### Response

200

application/json

Incentive programs retrieved successfully

[​](#response-incentive-programs)

incentive\_programs

object\[\]

required

Showchild attributes

[​](#response-next-cursor)

next\_cursor

string

Cursor for pagination to get the next page of results


---

###### Get Live Data - API DocumentationGet Live Data

#### Path Parameters

[​](#parameter-type)

type

string

required

Type of live data

[​](#parameter-milestone-id)

milestone\_id

string

required

Milestone ID

#### Response

200

application/json

Live data retrieved successfully

[​](#response-live-data)

live\_data

object

required

Showchild attributes


---

###### Get Multiple Live Data - API DocumentationGet Multiple Live Data

#### Query Parameters

[​](#parameter-milestone-ids)

milestone\_ids

string\[\]

required

Array of milestone IDs

Maximum array length: `100`

#### Response

200

application/json

Live data retrieved successfully

[​](#response-live-datas)

live\_datas

object\[\]

required

Showchild attributes


---

###### Batch Get Market Candlesticks - API DocumentationBatch Get Market Candlesticks

#### Query Parameters

[​](#parameter-market-tickers)

market\_tickers

string

required

Comma-separated list of market tickers (maximum 100)

[​](#parameter-start-ts)

start\_ts

integer<int64>

required

Start timestamp in Unix seconds

[​](#parameter-end-ts)

end\_ts

integer<int64>

required

End timestamp in Unix seconds

[​](#parameter-period-interval)

period\_interval

integer<int32>

required

Candlestick period interval in minutes

Required range: `x >= 1`

[​](#parameter-include-latest-before-start)

include\_latest\_before\_start

boolean

default:false

If true, prepends the latest candlestick available before the start\_ts. This synthetic candlestick is created by:

1. Finding the most recent real candlestick before start\_ts
2. Projecting it forward to the first period boundary (calculated as the next period interval after start\_ts)
3. Setting all OHLC prices to null, and `previous_price` to the close price from the real candlestick

#### Response

200

application/json

Market candlesticks retrieved successfully

[​](#response-markets)

markets

object\[\]

required

Array of market candlestick data, one entry per requested market.

Showchild attributes


---

###### Get Market - API DocumentationGet Market

#### Path Parameters

[​](#parameter-ticker)

ticker

string

required

Market ticker

#### Response

200

application/json

Market retrieved successfully

[​](#response-market)

market

object

required

Showchild attributes


---

###### Get Market Candlesticks - API DocumentationGet Market Candlesticks

#### Path Parameters

[​](#parameter-series-ticker)

series\_ticker

string

required

Series ticker - the series that contains the target market

[​](#parameter-ticker)

ticker

string

required

Market ticker - unique identifier for the specific market

#### Query Parameters

[​](#parameter-start-ts)

start\_ts

integer<int64>

required

Start timestamp (Unix timestamp). Candlesticks will include those ending on or after this time.

[​](#parameter-end-ts)

end\_ts

integer<int64>

required

End timestamp (Unix timestamp). Candlesticks will include those ending on or before this time.

[​](#parameter-period-interval)

period\_interval

enum<integer>

required

Time period length of each candlestick in minutes. Valid values are 1 (1 minute), 60 (1 hour), or 1440 (1 day).

Available options:

`1`,

`60`,

`1440`

#### Response

200

application/json

Candlesticks retrieved successfully

[​](#response-ticker)

ticker

string

required

Unique identifier for the market.

[​](#response-candlesticks)

candlesticks

object\[\]

required

Array of candlestick data points for the specified time range.

Showchild attributes


---

###### Get Market Orderbook - API DocumentationGet Market Orderbook

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-ticker)

ticker

string

required

Market ticker

#### Query Parameters

[​](#parameter-depth)

depth

integer

default:0

Depth of the orderbook to retrieve (0 or negative means all levels, 1-100 for specific depth)

Required range: `0 <= x <= 100`

#### Response

200

application/json

Orderbook retrieved successfully

[​](#response-orderbook)

orderbook

object

required

Showchild attributes


---

###### Get Markets - API DocumentationGet Markets

#### Query Parameters

[​](#parameter-limit)

limit

integer<int64>

default:100

Number of results per page. Defaults to 100. Maximum value is 1000.

Required range: `1 <= x <= 1000`

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).

[​](#parameter-series-ticker)

series\_ticker

string

Filter by series ticker

[​](#parameter-min-created-ts)

min\_created\_ts

integer<int64>

Filter items that created after this Unix timestamp

[​](#parameter-max-created-ts)

max\_created\_ts

integer<int64>

Filter items that created before this Unix timestamp

[​](#parameter-max-close-ts)

max\_close\_ts

integer<int64>

Filter items that close before this Unix timestamp

[​](#parameter-min-close-ts)

min\_close\_ts

integer<int64>

Filter items that close after this Unix timestamp

[​](#parameter-min-settled-ts)

min\_settled\_ts

integer<int64>

Filter items that settled after this Unix timestamp

[​](#parameter-max-settled-ts)

max\_settled\_ts

integer<int64>

Filter items that settled before this Unix timestamp

[​](#parameter-status)

status

enum<string>

Filter by market status. Leave empty to return markets with any status.

Available options:

`unopened`,

`open`,

`paused`,

`closed`,

`settled`

[​](#parameter-tickers)

tickers

string

Filter by specific market tickers. Comma-separated list of market tickers to retrieve.

[​](#parameter-mve-filter)

mve\_filter

enum<string>

Filter by multivariate events (combos). 'only' returns only multivariate events, 'exclude' excludes multivariate events.

Available options:

`only`,

`exclude`

#### Response

200

application/json

Markets retrieved successfully

[​](#response-markets)

markets

object\[\]

required

Showchild attributes

[​](#response-cursor)

cursor

string

required


---

###### Get Series - API DocumentationGet Series

#### Path Parameters

[​](#parameter-series-ticker)

series\_ticker

string

required

The ticker of the series to retrieve

#### Response

200

application/json

Series retrieved successfully

[​](#response-series)

series

object

required

Showchild attributes


---

###### Get Series List - API DocumentationGet Series List

#### Query Parameters

[​](#parameter-category)

category

string

[​](#parameter-tags)

tags

string

[​](#parameter-include-product-metadata)

include\_product\_metadata

boolean

default:false

#### Response

200

application/json

Series list retrieved successfully

[​](#response-series)

series

object\[\]

required

Showchild attributes


---

###### Get Trades - API DocumentationGet Trades

#### Query Parameters

[​](#parameter-limit)

limit

integer<int64>

default:100

Number of results per page. Defaults to 100. Maximum value is 1000.

Required range: `1 <= x <= 1000`

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

[​](#parameter-ticker)

ticker

string

Filter by market ticker

[​](#parameter-min-ts)

min\_ts

integer<int64>

Filter items after this Unix timestamp

[​](#parameter-max-ts)

max\_ts

integer<int64>

Filter items before this Unix timestamp

#### Response

200

application/json

Trades retrieved successfully

[​](#response-trades)

trades

object\[\]

required

Showchild attributes

[​](#response-cursor)

cursor

string

required


---

###### Get Milestone - API DocumentationGet Milestone

#### Path Parameters

[​](#parameter-milestone-id)

milestone\_id

string

required

Milestone ID

#### Response

200

application/json

Milestone retrieved successfully

[​](#response-milestone)

milestone

object

required

The milestone data.

Showchild attributes


---

###### Get Milestones - API DocumentationGet Milestones

#### Query Parameters

[​](#parameter-limit)

limit

integer

required

Number of milestones to return per page

Required range: `1 <= x <= 500`

[​](#parameter-minimum-start-date)

minimum\_start\_date

string<date-time>

Minimum start date to filter milestones. Format RFC3339 timestamp

[​](#parameter-category)

category

string

Filter by milestone category

[​](#parameter-competition)

competition

string

Filter by competition

[​](#parameter-source-id)

source\_id

string

Filter by source id

[​](#parameter-type)

type

string

Filter by milestone type

[​](#parameter-related-event-ticker)

related\_event\_ticker

string

Filter by related event ticker

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results

#### Response

200

application/json

Milestones retrieved successfully

[​](#response-milestones)

milestones

object\[\]

required

List of milestones.

Showchild attributes

[​](#response-cursor)

cursor

string

Cursor for pagination.


---

###### Create Market In Multivariate Event Collection - API DocumentationCreate Market In Multivariate Event Collection

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-collection-ticker)

collection\_ticker

string

required

Collection ticker

#### Body

application/json

[​](#body-selected-markets)

selected\_markets

object\[\]

required

List of selected markets that act as parameters to determine which market is created.

Showchild attributes

#### Response

200

application/json

Market created successfully

[​](#response-event-ticker)

event\_ticker

string

required

Event ticker for the created market.

[​](#response-market-ticker)

market\_ticker

string

required

Market ticker for the created market.


---

###### Get Multivariate Event Collection - API DocumentationGet Multivariate Event Collection

#### Path Parameters

[​](#parameter-collection-ticker)

collection\_ticker

string

required

Collection ticker

#### Response

200

application/json

Collection retrieved successfully

[​](#response-multivariate-contract)

multivariate\_contract

object

required

The multivariate event collection.

Showchild attributes


---

###### Get Multivariate Event Collection Lookup History - API DocumentationGet Multivariate Event Collection Lookup History

#### Path Parameters

[​](#parameter-collection-ticker)

collection\_ticker

string

required

Collection ticker

#### Query Parameters

[​](#parameter-lookback-seconds)

lookback\_seconds

enum<integer>

required

Number of seconds to look back for lookup history. Must be one of 10, 60, 300, or 3600.

Available options:

`10`,

`60`,

`300`,

`3600`

#### Response

200

application/json

Lookup history retrieved successfully

[​](#response-lookup-points)

lookup\_points

object\[\]

required

List of recent lookup points in the collection.

Showchild attributes


---

###### Get Multivariate Event Collections - API DocumentationGet Multivariate Event Collections

#### Query Parameters

[​](#parameter-status)

status

enum<string>

Only return collections of a certain status. Can be unopened, open, or closed.

Available options:

`unopened`,

`open`,

`closed`

[​](#parameter-associated-event-ticker)

associated\_event\_ticker

string

Only return collections associated with a particular event ticker.

[​](#parameter-series-ticker)

series\_ticker

string

Only return collections with a particular series ticker.

[​](#parameter-limit)

limit

integer<int32>

Specify the maximum number of results.

Required range: `1 <= x <= 200`

[​](#parameter-cursor)

cursor

string

The Cursor represents a pointer to the next page of records in the pagination. This optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point.

#### Response

200

application/json

Collections retrieved successfully

[​](#response-multivariate-contracts)

multivariate\_contracts

object\[\]

required

List of multivariate event collections.

Showchild attributes

[​](#response-cursor)

cursor

string

The Cursor represents a pointer to the next page of records in the pagination. Use the value returned here in the cursor query parameter for this end-point to get the next page containing limit records. An empty value of this field indicates there is no next page.


---

###### Lookup Tickers For Market In Multivariate Event Collection - API DocumentationLookup Tickers For Market In Multivariate Event Collection

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-collection-ticker)

collection\_ticker

string

required

Collection ticker

#### Body

application/json

[​](#body-selected-markets)

selected\_markets

object\[\]

required

List of selected markets that act as parameters to determine which market is produced.

Showchild attributes

#### Response

200

application/json

Market looked up successfully

[​](#response-event-ticker)

event\_ticker

string

required

Event ticker for the looked up market.

[​](#response-market-ticker)

market\_ticker

string

required

Market ticker for the looked up market.


---

###### Create Order Group - API DocumentationCreate Order Group

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-contracts-limit)

contracts\_limit

integer<int64>

required

Specifies the maximum number of contracts that can be matched within this group.

Required range: `x >= 1`

#### Response

201

application/json

Order group created successfully

[​](#response-order-group-id)

order\_group\_id

string

required

The unique identifier for the created order group


---

###### Delete Order Group - API DocumentationDelete Order Group

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-group-id)

order\_group\_id

string

required

Order group ID

#### Response

200

application/json

Order group deleted successfully

An empty response body


---

###### Get Order Group - API DocumentationGet Order Group

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-group-id)

order\_group\_id

string

required

Order group ID

#### Response

200

application/json

Order group retrieved successfully

[​](#response-is-auto-cancel-enabled)

is\_auto\_cancel\_enabled

boolean

required

Whether auto-cancel is enabled for this order group

[​](#response-orders)

orders

string\[\]

required

List of order IDs that belong to this order group


---

###### Get Order Groups - API DocumentationGet Order Groups

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Response

200

application/json

Order groups retrieved successfully

[​](#response-order-groups)

order\_groups

object\[\]

Showchild attributes


---

###### Reset Order Group - API DocumentationReset Order Group

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-group-id)

order\_group\_id

string

required

Order group ID

#### Body

application/json

An empty response body

#### Response

200

application/json

Order group reset successfully

An empty response body


---

###### Amend Order - API DocumentationAmend Order

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-id)

order\_id

string

required

Order ID

#### Body

application/json

[​](#body-ticker)

ticker

string

required

Market ticker

[​](#body-side)

side

enum<string>

required

Side of the order

Available options:

`yes`,

`no`

[​](#body-action)

action

enum<string>

required

Action of the order

Available options:

`buy`,

`sell`

[​](#body-client-order-id)

client\_order\_id

string

required

The original client-specified order ID to be amended

[​](#body-updated-client-order-id)

updated\_client\_order\_id

string

required

The new client-specified order ID after amendment

[​](#body-yes-price)

yes\_price

integer

Updated yes price for the order in cents

Required range: `1 <= x <= 99`

[​](#body-no-price)

no\_price

integer

Updated no price for the order in cents

Required range: `1 <= x <= 99`

[​](#body-yes-price-dollars)

yes\_price\_dollars

string

Updated yes price for the order in fixed-point dollars. Exactly one of yes\_price, no\_price, yes\_price\_dollars, and no\_price\_dollars must be passed.

Example:

`"0.5600"`

[​](#body-no-price-dollars)

no\_price\_dollars

string

Updated no price for the order in fixed-point dollars. Exactly one of yes\_price, no\_price, yes\_price\_dollars, and no\_price\_dollars must be passed.

Example:

`"0.5600"`

[​](#body-count)

count

integer

Updated quantity for the order

Required range: `x >= 1`

#### Response

200

application/json

Order amended successfully

[​](#response-old-order)

old\_order

object

required

The order before amendment

Showchild attributes

[​](#response-order)

order

object

required

The order after amendment

Showchild attributes


---

###### Batch Cancel Orders - API DocumentationBatch Cancel Orders

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-ids)

ids

string\[\]

required

An array of order IDs to cancel

#### Response

200

application/json

Batch order cancellation completed

[​](#response-orders)

orders

object\[\]

required

Showchild attributes


---

###### Batch Create Orders - API DocumentationBatch Create Orders

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-orders)

orders

object\[\]

required

Showchild attributes

#### Response

201

application/json

Batch order creation completed

[​](#response-orders)

orders

object\[\]

required

Showchild attributes


---

###### Cancel Order - API DocumentationCancel Order

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-id)

order\_id

string

required

Order ID

#### Response

200

application/json

Order cancelled successfully

[​](#response-order)

order

object

required

Showchild attributes

[​](#response-reduced-by)

reduced\_by

integer

required


---

###### Create Order - API DocumentationCreate Order

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Body

application/json

[​](#body-ticker)

ticker

string

required

[​](#body-side)

side

enum<string>

required

Available options:

`yes`,

`no`

[​](#body-action)

action

enum<string>

required

Available options:

`buy`,

`sell`

[​](#body-count)

count

integer

required

Required range: `x >= 1`

[​](#body-client-order-id)

client\_order\_id

string

[​](#body-type)

type

enum<string>

Available options:

`limit`,

`market`

[​](#body-yes-price)

yes\_price

integer

Required range: `1 <= x <= 99`

[​](#body-no-price)

no\_price

integer

Required range: `1 <= x <= 99`

[​](#body-yes-price-dollars)

yes\_price\_dollars

string

Submitting price of the Yes side in fixed-point dollars

Example:

`"0.5600"`

[​](#body-no-price-dollars)

no\_price\_dollars

string

Submitting price of the No side in fixed-point dollars

Example:

`"0.5600"`

[​](#body-expiration-ts)

expiration\_ts

integer<int64>

[​](#body-time-in-force)

time\_in\_force

enum<string>

Available options:

`fill_or_kill`,

`good_till_canceled`,

`immediate_or_cancel`

[​](#body-buy-max-cost)

buy\_max\_cost

integer

Maximum cost in cents. When specified, the order will automatically have Fill-or-Kill (FoK) behavior.

[​](#body-post-only)

post\_only

boolean

[​](#body-reduce-only)

reduce\_only

boolean

[​](#body-sell-position-floor)

sell\_position\_floor

integer

Deprecated: Use reduce\_only instead. Only accepts value of 0.

[​](#body-self-trade-prevention-type)

self\_trade\_prevention\_type

enum<string>

The self-trade prevention type for orders

Available options:

`taker_at_cross`,

`maker`

[​](#body-order-group-id)

order\_group\_id

string

The order group this order is part of

[​](#body-cancel-order-on-pause)

cancel\_order\_on\_pause

boolean

If this flag is set to true, the order will be canceled if the order is open and trading on the exchange is paused for any reason.

#### Response

201

application/json

Order created successfully

[​](#response-order)

order

object

required

Showchild attributes


---

###### Decrease Order - API DocumentationDecrease Order

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-id)

order\_id

string

required

Order ID

#### Body

application/json

[​](#body-reduce-by)

reduce\_by

integer

Required range: `x >= 1`

[​](#body-reduce-to)

reduce\_to

integer

Required range: `x >= 0`

#### Response

200

application/json

Order decreased successfully

[​](#response-order)

order

object

required

Showchild attributes


---

###### Get Order - API DocumentationGet Order

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-id)

order\_id

string

required

Order ID

#### Response

200

application/json

Order retrieved successfully

[​](#response-order)

order

object

required

Showchild attributes


---

###### Get Order Queue Position - API DocumentationGet Order Queue Position

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Path Parameters

[​](#parameter-order-id)

order\_id

string

required

Order ID

#### Response

200

application/json

Queue position retrieved successfully

[​](#response-queue-position)

queue\_position

integer<int32>

required

The position of the order in the queue


---

###### Get Orders - API DocumentationGet Orders

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-ticker)

ticker

string

Filter by market ticker

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).

[​](#parameter-min-ts)

min\_ts

integer<int64>

Filter items after this Unix timestamp

[​](#parameter-max-ts)

max\_ts

integer<int64>

Filter items before this Unix timestamp

[​](#parameter-status)

status

string

Filter by status. Possible values depend on the endpoint.

[​](#parameter-limit)

limit

integer<int64>

default:100

Number of results per page. Defaults to 100. Maximum value is 200.

Required range: `1 <= x <= 200`

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

#### Response

200

application/json

Orders retrieved successfully

[​](#response-orders)

orders

object\[\]

required

Showchild attributes

[​](#response-cursor)

cursor

string

required


---

###### Get Queue Positions for Orders - API DocumentationGet Queue Positions for Orders

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-market-tickers)

market\_tickers

string

Comma-separated list of market tickers to filter by

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker to filter by

#### Response

200

application/json

Queue positions retrieved successfully

[​](#response-queue-positions)

queue\_positions

object\[\]

required

Queue positions for all matching orders

Showchild attributes


---

###### Get Balance - API DocumentationGet Balance

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Response

200

application/json

Balance retrieved successfully

[​](#response-balance)

balance

integer<int64>

required

Member's available balance in cents. This represents the amount available for trading.

[​](#response-portfolio-value)

portfolio\_value

integer<int64>

required

Member's portfolio value in cents. This is the current value of all positions held.

[​](#response-updated-ts)

updated\_ts

integer<int64>

required

Unix timestamp of the last update to the balance.


---

###### Get Fills - API DocumentationGet Fills

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-ticker)

ticker

string

Filter by market ticker

[​](#parameter-order-id)

order\_id

string

Filter by order ID

[​](#parameter-min-ts)

min\_ts

integer<int64>

Filter items after this Unix timestamp

[​](#parameter-max-ts)

max\_ts

integer<int64>

Filter items before this Unix timestamp

[​](#parameter-limit)

limit

integer<int64>

default:100

Number of results per page. Defaults to 100. Maximum value is 200.

Required range: `1 <= x <= 200`

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

#### Response

200

application/json

Fills retrieved successfully

[​](#response-fills)

fills

object\[\]

required

Showchild attributes

[​](#response-cursor)

cursor

string

required


---

###### Get Positions - API DocumentationGet Positions

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-cursor)

cursor

string

The Cursor represents a pointer to the next page of records in the pagination. Use the value returned from the previous response to get the next page.

[​](#parameter-limit)

limit

integer<int32>

default:100

Parameter to specify the number of results per page. Defaults to 100.

Required range: `1 <= x <= 1000`

[​](#parameter-count-filter)

count\_filter

string

Restricts the positions to those with any of following fields with non-zero values, as a comma separated list. The following values are accepted - position, total\_traded

[​](#parameter-ticker)

ticker

string

Filter by market ticker

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).

#### Response

200

application/json

Positions retrieved successfully

[​](#response-market-positions)

market\_positions

object\[\]

required

List of market positions

Showchild attributes

[​](#response-event-positions)

event\_positions

object\[\]

required

List of event positions

Showchild attributes

[​](#response-cursor)

cursor

string

The Cursor represents a pointer to the next page of records in the pagination. Use the value returned here in the cursor query parameter for this end-point to get the next page containing limit records. An empty value of this field indicates there is no next page.


---

###### Get Settlements - API DocumentationGet Settlements

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Query Parameters

[​](#parameter-limit)

limit

integer<int64>

default:100

Number of results per page. Defaults to 100. Maximum value is 200.

Required range: `1 <= x <= 200`

[​](#parameter-cursor)

cursor

string

Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page.

[​](#parameter-ticker)

ticker

string

Filter by market ticker

[​](#parameter-event-ticker)

event\_ticker

string

Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).

[​](#parameter-min-ts)

min\_ts

integer<int64>

Filter items after this Unix timestamp

[​](#parameter-max-ts)

max\_ts

integer<int64>

Filter items before this Unix timestamp

#### Response

200

application/json

Settlements retrieved successfully

[​](#response-settlements)

settlements

object\[\]

required

Showchild attributes

[​](#response-cursor)

cursor

string


---

###### Get Total Resting Order Value - API DocumentationGet Total Resting Order Value

#### Authorizations

[​](#authorization-kalshi-access-key)

KALSHI-ACCESS-KEY

string

header

required

Your API key ID

[​](#authorization-kalshi-access-signature)

KALSHI-ACCESS-SIGNATURE

string

header

required

RSA-PSS signature of the request

[​](#authorization-kalshi-access-timestamp)

KALSHI-ACCESS-TIMESTAMP

string

header

required

Request timestamp in milliseconds

#### Response

200

application/json

Total resting order value retrieved successfully

[​](#response-total-resting-order-value)

total\_resting\_order\_value

integer

required

Total value of resting orders in cents


---

###### Get Filters for Sports - API DocumentationGet Filters for Sports

#### Response

200

application/json

Filters retrieved successfully

[​](#response-filters-by-sports)

filters\_by\_sports

object

required

Mapping of sports to their filter details

Showchild attributes

[​](#response-sport-ordering)

sport\_ordering

string\[\]

required

Ordered list of sports for display


---

###### Get Tags for Series Categories - API DocumentationGet Tags for Series Categories

#### Response

200

application/json

Tags retrieved successfully

[​](#response-tags-by-categories)

tags\_by\_categories

object

required

Mapping of series categories to their associated tags

Showchild attributes


---

###### Get Structured Target - API DocumentationGet Structured Target

#### Path Parameters

[​](#parameter-structured-target-id)

structured\_target\_id

string

required

Structured target ID

#### Response

200

application/json

Structured target retrieved successfully

[​](#response-structured-target)

structured\_target

object

Showchild attributes


---

###### Get Structured Targets - API DocumentationGet Structured Targets

#### Query Parameters

[​](#parameter-type)

type

string

Filter by structured target type

[​](#parameter-competition)

competition

string

Filter by competition

[​](#parameter-page-size)

page\_size

integer<int32>

default:100

Number of items per page (min 1, max 2000, default 100)

Required range: `1 <= x <= 2000`

[​](#parameter-cursor)

cursor

string

Pagination cursor

#### Response

200

application/json

Structured targets retrieved successfully

[​](#response-structured-targets)

structured\_targets

object\[\]

Showchild attributes

[​](#response-cursor)

cursor

string

Pagination cursor for the next page. Empty if there are no more results.
