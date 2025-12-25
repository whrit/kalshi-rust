## Table of Contents

  - [Making Your First Request - API DocumentationMaking Your First RequestTo make your request we recommend testing a public endpoint like GetMarkets. As you explore our other endpoints, you’ll notice some endpoints return an authentication_error. If you want to experiment with these endpoints, you will need to get API Keys. You may also want to sign up for a demo account to test without real funds. The following resources might help you on your journey to exploring Kalshi’s markets: Python starter code bare-bones API interactions with authentication examples (also see API Keys) Python starter code 2 this Python client is not maintained, but you may still find it useful as a reference for how to call certain endpoints Discord and check out #dev and #support](#making-your-first-request-api-documentationmaking-your-first-requestto-make-your-request-we-recommend-testing-a-public-endpoint-like-getmarkets-as-you-explore-our-other-endpoints-you’ll-notice-some-endpoints-return-an-authentication_error-if-you-want-to-experiment-with-these-endpoints-you-will-need-to-get-api-keys-you-may-also-want-to-sign-up-for-a-demo-account-to-test-without-real-funds-the-following-resources-might-help-you-on-your-journey-to-exploring-kalshi’s-markets-python-starter-code-bare-bones-api-interactions-with-authentication-examples-also-see-api-keys-python-starter-code-2-this-python-client-is-not-maintained-but-you-may-still-find-it-useful-as-a-reference-for-how-to-call-certain-endpoints-discord-and-check-out-dev-and-support)
    - [Quick Start: Market Data - API DocumentationQuick Start: Market DataThis guide will walk you through accessing Kalshi’s public market data endpoints without authentication. You’ll learn how to retrieve series information, events, markets, and orderbook data for the popular “Who will have a higher net approval” market. ​Making Unauthenticated Requests Kalshi provides several public endpoints that don’t require API keys. These endpoints allow you to access market data directly from our production servers at https://api.elections.kalshi.com/trade-api/v2. Note about the API URL: Despite the “elections” subdomain, api.elections.kalshi.com provides access to ALL Kalshi markets - not just election-related ones. This includes markets on economics, climate, technology, entertainment, and more. No authentication headers are required for the endpoints in this guide. You can start making requests immediately! ​Step 1: Get Series Information Let’s start by fetching information about the KXHIGHNY series (Highest temperature in NYC today?). This series tracks the highest temperature recorded in Central Park, New York on a given day. We’ll use the Get Series endpoint. PythonJavaScriptcURLCopyimport requests # Get series information for KXHIGHNY url = "https://api.elections.kalshi.com/trade-api/v2/series/KXHIGHNY" response = requests.get(url) series_data = response.json() print(f"Series Title: {series_data['series']['title']}") print(f"Frequency: {series_data['series']['frequency']}") print(f"Category: {series_data['series']['category']}") ​Step 2: Get Today’s Events and Markets Now that we have the series information, let’s get the markets for this series. We’ll use the Get Markets endpoint with the series ticker filter to find all active markets. PythonJavaScriptCopy# Get all markets for the KXHIGHNY series markets_url = f"https://api.elections.kalshi.com/trade-api/v2/markets?series_ticker=KXHIGHNY&status=open" markets_response = requests.get(markets_url) markets_data = markets_response.json() print(f"\nActive markets in KXHIGHNY series:") for market in markets_data['markets']: print(f"- {market['ticker']}: {market['title']}") print(f" Event: {market['event_ticker']}") print(f" Yes Price: {market['yes_price']}¢ | Volume: {market['volume']}") print() # Get details for a specific event if you have its ticker if markets_data['markets']: # Let's get details for the first market's event event_ticker = markets_data['markets'][0]['event_ticker'] event_url = f"https://api.elections.kalshi.com/trade-api/v2/events/{event_ticker}" event_response = requests.get(event_url) event_data = event_response.json() print(f"Event Details:") print(f"Title: {event_data['event']['title']}") print(f"Category: {event_data['event']['category']}") You can view these markets in the Kalshi UI at: https://kalshi.com/markets/kxhighny ​Step 3: Get Orderbook Data Now let’s fetch the orderbook for a specific market to see the current bids and asks using the Get Market Orderbook endpoint. PythonJavaScriptCopy# Get orderbook for a specific market # Replace with an actual market ticker from the markets list market_ticker = markets_data['markets'][0]['ticker'] orderbook_url = f"https://api.elections.kalshi.com/trade-api/v2/markets/{market_ticker}/orderbook" orderbook_response = requests.get(orderbook_url) orderbook_data = orderbook_response.json() print(f"\nOrderbook for {market_ticker}:") print("YES BIDS:") for bid in orderbook_data['orderbook']['yes'][:5]: # Show top 5 print(f" Price: {bid[0]}¢, Quantity: {bid[1]}") print("\nNO BIDS:") for bid in orderbook_data['orderbook']['no'][:5]: # Show top 5 print(f" Price: {bid[0]}¢, Quantity: {bid[1]}") ​Working with Large Datasets The Kalshi API uses cursor-based pagination to handle large datasets efficiently. To learn more about navigating through paginated responses, see our Understanding Pagination guide. ​Understanding Orderbook Responses Kalshi’s orderbook structure is unique due to the nature of binary prediction markets. The API only returns bids (not asks) because of the reciprocal relationship between YES and NO positions. To learn more about orderbook responses and why they work this way, see our Orderbook Responses guide. ​Next Steps Now that you understand how to access market data without authentication, you can: Explore other public series and events Build real-time market monitoring tools Create market analysis dashboards Set up a WebSocket connection for live updates (requires authentication) For authenticated endpoints that allow trading and portfolio management, check out our API Keys guide.](#quick-start-market-data-api-documentationquick-start-market-datathis-guide-will-walk-you-through-accessing-kalshi’s-public-market-data-endpoints-without-authentication-you’ll-learn-how-to-retrieve-series-information-events-markets-and-orderbook-data-for-the-popular-“who-will-have-a-higher-net-approval”-market-​making-unauthenticated-requests-kalshi-provides-several-public-endpoints-that-don’t-require-api-keys-these-endpoints-allow-you-to-access-market-data-directly-from-our-production-servers-at-httpsapielectionskalshicomtrade-apiv2-note-about-the-api-url-despite-the-“elections”-subdomain-apielectionskalshicom-provides-access-to-all-kalshi-markets-not-just-election-related-ones-this-includes-markets-on-economics-climate-technology-entertainment-and-more-no-authentication-headers-are-required-for-the-endpoints-in-this-guide-you-can-start-making-requests-immediately-​step-1-get-series-information-let’s-start-by-fetching-information-about-the-kxhighny-series-highest-temperature-in-nyc-today-this-series-tracks-the-highest-temperature-recorded-in-central-park-new-york-on-a-given-day-we’ll-use-the-get-series-endpoint-pythonjavascriptcurlcopyimport-requests-get-series-information-for-kxhighny-url-httpsapielectionskalshicomtrade-apiv2serieskxhighny-response-requestsgeturl-series_data-responsejson-printfseries-title-series_dataseriestitle-printffrequency-series_dataseriesfrequency-printfcategory-series_dataseriescategory-​step-2-get-today’s-events-and-markets-now-that-we-have-the-series-information-let’s-get-the-markets-for-this-series-we’ll-use-the-get-markets-endpoint-with-the-series-ticker-filter-to-find-all-active-markets-pythonjavascriptcopy-get-all-markets-for-the-kxhighny-series-markets_url-fhttpsapielectionskalshicomtrade-apiv2marketsseries_tickerkxhighnystatusopen-markets_response-requestsgetmarkets_url-markets_data-markets_responsejson-printfnactive-markets-in-kxhighny-series-for-market-in-markets_datamarkets-printf-marketticker-markettitle-printf-event-marketevent_ticker-printf-yes-price-marketyes_price¢-volume-marketvolume-print-get-details-for-a-specific-event-if-you-have-its-ticker-if-markets_datamarkets-lets-get-details-for-the-first-markets-event-event_ticker-markets_datamarkets0event_ticker-event_url-fhttpsapielectionskalshicomtrade-apiv2eventsevent_ticker-event_response-requestsgetevent_url-event_data-event_responsejson-printfevent-details-printftitle-event_dataeventtitle-printfcategory-event_dataeventcategory-you-can-view-these-markets-in-the-kalshi-ui-at-httpskalshicommarketskxhighny-​step-3-get-orderbook-data-now-let’s-fetch-the-orderbook-for-a-specific-market-to-see-the-current-bids-and-asks-using-the-get-market-orderbook-endpoint-pythonjavascriptcopy-get-orderbook-for-a-specific-market-replace-with-an-actual-market-ticker-from-the-markets-list-market_ticker-markets_datamarkets0ticker-orderbook_url-fhttpsapielectionskalshicomtrade-apiv2marketsmarket_tickerorderbook-orderbook_response-requestsgetorderbook_url-orderbook_data-orderbook_responsejson-printfnorderbook-for-market_ticker-printyes-bids-for-bid-in-orderbook_dataorderbookyes5-show-top-5-printf-price-bid0¢-quantity-bid1-printnno-bids-for-bid-in-orderbook_dataorderbookno5-show-top-5-printf-price-bid0¢-quantity-bid1-​working-with-large-datasets-the-kalshi-api-uses-cursor-based-pagination-to-handle-large-datasets-efficiently-to-learn-more-about-navigating-through-paginated-responses-see-our-understanding-pagination-guide-​understanding-orderbook-responses-kalshi’s-orderbook-structure-is-unique-due-to-the-nature-of-binary-prediction-markets-the-api-only-returns-bids-not-asks-because-of-the-reciprocal-relationship-between-yes-and-no-positions-to-learn-more-about-orderbook-responses-and-why-they-work-this-way-see-our-orderbook-responses-guide-​next-steps-now-that-you-understand-how-to-access-market-data-without-authentication-you-can-explore-other-public-series-and-events-build-real-time-market-monitoring-tools-create-market-analysis-dashboards-set-up-a-websocket-connection-for-live-updates-requires-authentication-for-authenticated-endpoints-that-allow-trading-and-portfolio-management-check-out-our-api-keys-guide)
      - [Understanding Pagination - API DocumentationUnderstanding PaginationThe Kalshi API uses cursor-based pagination to help you efficiently navigate through large datasets. This guide explains how pagination works and provides examples for handling paginated responses. ​How Pagination Works When making requests to list endpoints (like /markets, /events, or /series), the API returns results in pages to keep response sizes manageable. Each page contains: Data array: The actual items for the current page (markets, events, etc.) Cursor field: A token that points to the next page of results Limit: The maximum number of items per page (default: 100) ​Using Cursors To paginate through results: Make your initial request without a cursor Check if the response includes a cursor field If a cursor exists, make another request with ?cursor={cursor_value} Continue until the cursor is null (no more pages) ​Example: Paginating Through Markets PythonJavaScriptCopyimport requests def get_all_markets(series_ticker): """Fetch all markets for a series, handling pagination""" all_markets = [] cursor = None base_url = "https://api.elections.kalshi.com/trade-api/v2/markets" while True: # Build URL with cursor if we have one url = f"{base_url}?series_ticker={series_ticker}&limit=100" if cursor: url += f"&cursor={cursor}" response = requests.get(url) data = response.json() # Add markets from this page all_markets.extend(data['markets']) # Check if there are more pages cursor = data.get('cursor') if not cursor: break print(f"Fetched {len(data['markets'])} markets, total: {len(all_markets)}") return all_markets # Example usage markets = get_all_markets("KXHIGHNY") print(f"Total markets found: {len(markets)}") ​Pagination Parameters Most list endpoints support these pagination parameters: cursor: Token from previous response to get the next page limit: Number of items per page (typically 1-100, default: 100) ​Best Practices Handle rate limits: When paginating through large datasets, be mindful of rate limits Set appropriate limits: Use smaller page sizes if you only need a few items Cache results: Store paginated data locally to avoid repeated API calls Check for changes: Data can change between requests, so consider implementing refresh logic ​Endpoints Supporting Pagination The following endpoints support cursor-based pagination: Get Markets - /markets Get Events - /events Get Series - /series Get Trades - /markets/trades Get Portfolio History - /portfolio/history Get Fills - /portfolio/fills Get Orders - /portfolio/orders ​Common Patterns ​Fetching Recent Items If you only need recent items, you can limit results without pagination: Copy# Get just the 10 most recent markets url = "https://api.elections.kalshi.com/trade-api/v2/markets?limit=10&status=open" ​Filtering While Paginating You can combine filters with pagination: Copy# Get all open markets for a series url = f"{base_url}?series_ticker={ticker}&status=open&limit=100&cursor={cursor}" ​Detecting New Items To check for new items since your last fetch: Store the first item’s ID or timestamp from your previous fetch Paginate through results until you find that item Everything before it is new ​Next Steps Now that you understand pagination, you can efficiently work with large datasets in the Kalshi API. For more details on specific endpoints, check the API Reference.](#understanding-pagination-api-documentationunderstanding-paginationthe-kalshi-api-uses-cursor-based-pagination-to-help-you-efficiently-navigate-through-large-datasets-this-guide-explains-how-pagination-works-and-provides-examples-for-handling-paginated-responses-​how-pagination-works-when-making-requests-to-list-endpoints-like-markets-events-or-series-the-api-returns-results-in-pages-to-keep-response-sizes-manageable-each-page-contains-data-array-the-actual-items-for-the-current-page-markets-events-etc-cursor-field-a-token-that-points-to-the-next-page-of-results-limit-the-maximum-number-of-items-per-page-default-100-​using-cursors-to-paginate-through-results-make-your-initial-request-without-a-cursor-check-if-the-response-includes-a-cursor-field-if-a-cursor-exists-make-another-request-with-cursorcursor_value-continue-until-the-cursor-is-null-no-more-pages-​example-paginating-through-markets-pythonjavascriptcopyimport-requests-def-get_all_marketsseries_ticker-fetch-all-markets-for-a-series-handling-pagination-all_markets-cursor-none-base_url-httpsapielectionskalshicomtrade-apiv2markets-while-true-build-url-with-cursor-if-we-have-one-url-fbase_urlseries_tickerseries_tickerlimit100-if-cursor-url-fcursorcursor-response-requestsgeturl-data-responsejson-add-markets-from-this-page-all_marketsextenddatamarkets-check-if-there-are-more-pages-cursor-datagetcursor-if-not-cursor-break-printffetched-lendatamarkets-markets-total-lenall_markets-return-all_markets-example-usage-markets-get_all_marketskxhighny-printftotal-markets-found-lenmarkets-​pagination-parameters-most-list-endpoints-support-these-pagination-parameters-cursor-token-from-previous-response-to-get-the-next-page-limit-number-of-items-per-page-typically-1-100-default-100-​best-practices-handle-rate-limits-when-paginating-through-large-datasets-be-mindful-of-rate-limits-set-appropriate-limits-use-smaller-page-sizes-if-you-only-need-a-few-items-cache-results-store-paginated-data-locally-to-avoid-repeated-api-calls-check-for-changes-data-can-change-between-requests-so-consider-implementing-refresh-logic-​endpoints-supporting-pagination-the-following-endpoints-support-cursor-based-pagination-get-markets-markets-get-events-events-get-series-series-get-trades-marketstrades-get-portfolio-history-portfoliohistory-get-fills-portfoliofills-get-orders-portfolioorders-​common-patterns-​fetching-recent-items-if-you-only-need-recent-items-you-can-limit-results-without-pagination-copy-get-just-the-10-most-recent-markets-url-httpsapielectionskalshicomtrade-apiv2marketslimit10statusopen-​filtering-while-paginating-you-can-combine-filters-with-pagination-copy-get-all-open-markets-for-a-series-url-fbase_urlseries_tickertickerstatusopenlimit100cursorcursor-​detecting-new-items-to-check-for-new-items-since-your-last-fetch-store-the-first-item’s-id-or-timestamp-from-your-previous-fetch-paginate-through-results-until-you-find-that-item-everything-before-it-is-new-​next-steps-now-that-you-understand-pagination-you-can-efficiently-work-with-large-datasets-in-the-kalshi-api-for-more-details-on-specific-endpoints-check-the-api-reference)
      - [Quick Start: Authenticated Requests - API DocumentationQuick Start: Authenticated RequestsThis guide shows you how to make authenticated requests to the Kalshi API in three simple steps. ​Step 1: Get Your API Keys Log in to your Kalshi account (demo or production) Navigate to Account & security → API Keys Click Create Key Save both: Private Key: Downloaded as a .key file API Key ID: Displayed on screen (looks like a952bcbe-ec3b-4b5b-b8f9-11dae589608c) Your private key cannot be retrieved after this page is closed. Store it securely! ​Step 2: Set Up Your Request Every authenticated request to Kalshi requires three headers: HeaderDescriptionExampleKALSHI-ACCESS-KEYYour API Key IDa952bcbe-ec3b-4b5b-b8f9-11dae589608cKALSHI-ACCESS-TIMESTAMPCurrent time in milliseconds1703123456789KALSHI-ACCESS-SIGNATURERequest signature (see below)base64_encoded_signature ​How to Create the Signature The signature proves you own the private key. Here’s how it works: Create a message string: Concatenate timestamp + HTTP_METHOD + path Example: 1703123456789GET/trade-api/v2/portfolio/balance Important: Use the path without query parameters. For /portfolio/orders?limit=5, sign only /trade-api/v2/portfolio/orders Sign with your private key: Use RSA-PSS with SHA256 Encode as base64: Convert the signature to base64 string Here’s the signing process in Python: Copyimport base64 from cryptography.hazmat.primitives import hashes from cryptography.hazmat.primitives.asymmetric import padding def sign_request(private_key, timestamp, method, path): # Strip query parameters from path before signing path_without_query = path.split('?')[0] # Create the message to sign message = f"{timestamp}{method}{path_without_query}".encode('utf-8') # Sign with RSA-PSS signature = private_key.sign( message, padding.PSS( mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH ), hashes.SHA256() ) # Return base64 encoded return base64.b64encode(signature).decode('utf-8') ​Step 3: Get Your Balance Now let’s make your first authenticated request to get your account balance: Copyimport requests import datetime # Set up the request timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) method = "GET" path = "/trade-api/v2/portfolio/balance" # Create signature (using function from Step 2) signature = sign_request(private_key, timestamp, method, path) # Make the request headers = { 'KALSHI-ACCESS-KEY': 'your-api-key-id', 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp } response = requests.get('https://demo-api.kalshi.co' + path, headers=headers) balance = response.json() print(f"Your balance: ${balance['balance'] / 100:.2f}") ​Complete Working Example Here’s the minimal code to get your balance: Copyimport requests import datetime import base64 from cryptography.hazmat.primitives import serialization, hashes from cryptography.hazmat.backends import default_backend from cryptography.hazmat.primitives.asymmetric import padding # Configuration API_KEY_ID = 'your-api-key-id-here' PRIVATE_KEY_PATH = 'path/to/your/kalshi-key.key' BASE_URL = 'https://demo-api.kalshi.co' # or 'https://api.kalshi.com' for production def load_private_key(key_path): """Load the private key from file.""" with open(key_path, "rb") as f: return serialization.load_pem_private_key(f.read(), password=None, backend=default_backend()) def create_signature(private_key, timestamp, method, path): """Create the request signature.""" # Strip query parameters before signing path_without_query = path.split('?')[0] message = f"{timestamp}{method}{path_without_query}".encode('utf-8') signature = private_key.sign( message, padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH), hashes.SHA256() ) return base64.b64encode(signature).decode('utf-8') def get(private_key, api_key_id, path, base_url=BASE_URL): """Make an authenticated GET request to the Kalshi API.""" timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) signature = create_signature(private_key, timestamp, "GET", path) headers = { 'KALSHI-ACCESS-KEY': api_key_id, 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp } return requests.get(base_url + path, headers=headers) # Load private key private_key = load_private_key(PRIVATE_KEY_PATH) # Get balance response = get(private_key, API_KEY_ID, "/trade-api/v2/portfolio/balance") print(f"Your balance: ${response.json()['balance'] / 100:.2f}") ​Common Issues ProblemSolution401 UnauthorizedCheck your API Key ID and private key file pathSignature errorEnsure timestamp is in milliseconds (not seconds)Path not foundPath must start with /trade-api/v2/Signature error with query paramsStrip query parameters before signing (use path.split('?')[0]) ​Next Steps Now you can make authenticated requests! Try these endpoints: /trade-api/v2/portfolio/positions - Get your positions /trade-api/v2/portfolio/orders - View your orders /trade-api/v2/markets - Browse available markets For more details, see the Complete Order Lifecycle guide or explore the API Reference.](#quick-start-authenticated-requests-api-documentationquick-start-authenticated-requeststhis-guide-shows-you-how-to-make-authenticated-requests-to-the-kalshi-api-in-three-simple-steps-​step-1-get-your-api-keys-log-in-to-your-kalshi-account-demo-or-production-navigate-to-account-security-→-api-keys-click-create-key-save-both-private-key-downloaded-as-a-key-file-api-key-id-displayed-on-screen-looks-like-a952bcbe-ec3b-4b5b-b8f9-11dae589608c-your-private-key-cannot-be-retrieved-after-this-page-is-closed-store-it-securely-​step-2-set-up-your-request-every-authenticated-request-to-kalshi-requires-three-headers-headerdescriptionexamplekalshi-access-keyyour-api-key-ida952bcbe-ec3b-4b5b-b8f9-11dae589608ckalshi-access-timestampcurrent-time-in-milliseconds1703123456789kalshi-access-signaturerequest-signature-see-belowbase64_encoded_signature-​how-to-create-the-signature-the-signature-proves-you-own-the-private-key-here’s-how-it-works-create-a-message-string-concatenate-timestamp-http_method-path-example-1703123456789gettrade-apiv2portfoliobalance-important-use-the-path-without-query-parameters-for-portfolioorderslimit5-sign-only-trade-apiv2portfolioorders-sign-with-your-private-key-use-rsa-pss-with-sha256-encode-as-base64-convert-the-signature-to-base64-string-here’s-the-signing-process-in-python-copyimport-base64-from-cryptographyhazmatprimitives-import-hashes-from-cryptographyhazmatprimitivesasymmetric-import-padding-def-sign_requestprivate_key-timestamp-method-path-strip-query-parameters-from-path-before-signing-path_without_query-pathsplit0-create-the-message-to-sign-message-ftimestampmethodpath_without_queryencodeutf-8-sign-with-rsa-pss-signature-private_keysign-message-paddingpss-mgfpaddingmgf1hashessha256-salt_lengthpaddingpssdigest_length-hashessha256-return-base64-encoded-return-base64b64encodesignaturedecodeutf-8-​step-3-get-your-balance-now-let’s-make-your-first-authenticated-request-to-get-your-account-balance-copyimport-requests-import-datetime-set-up-the-request-timestamp-strintdatetimedatetimenowtimestamp-1000-method-get-path-trade-apiv2portfoliobalance-create-signature-using-function-from-step-2-signature-sign_requestprivate_key-timestamp-method-path-make-the-request-headers-kalshi-access-key-your-api-key-id-kalshi-access-signature-signature-kalshi-access-timestamp-timestamp-response-requestsgethttpsdemo-apikalshico-path-headersheaders-balance-responsejson-printfyour-balance-balancebalance-1002f-​complete-working-example-here’s-the-minimal-code-to-get-your-balance-copyimport-requests-import-datetime-import-base64-from-cryptographyhazmatprimitives-import-serialization-hashes-from-cryptographyhazmatbackends-import-default_backend-from-cryptographyhazmatprimitivesasymmetric-import-padding-configuration-api_key_id-your-api-key-id-here-private_key_path-pathtoyourkalshi-keykey-base_url-httpsdemo-apikalshico-or-httpsapikalshicom-for-production-def-load_private_keykey_path-load-the-private-key-from-file-with-openkey_path-rb-as-f-return-serializationload_pem_private_keyfread-passwordnone-backenddefault_backend-def-create_signatureprivate_key-timestamp-method-path-create-the-request-signature-strip-query-parameters-before-signing-path_without_query-pathsplit0-message-ftimestampmethodpath_without_queryencodeutf-8-signature-private_keysign-message-paddingpssmgfpaddingmgf1hashessha256-salt_lengthpaddingpssdigest_length-hashessha256-return-base64b64encodesignaturedecodeutf-8-def-getprivate_key-api_key_id-path-base_urlbase_url-make-an-authenticated-get-request-to-the-kalshi-api-timestamp-strintdatetimedatetimenowtimestamp-1000-signature-create_signatureprivate_key-timestamp-get-path-headers-kalshi-access-key-api_key_id-kalshi-access-signature-signature-kalshi-access-timestamp-timestamp-return-requestsgetbase_url-path-headersheaders-load-private-key-private_key-load_private_keyprivate_key_path-get-balance-response-getprivate_key-api_key_id-trade-apiv2portfoliobalance-printfyour-balance-responsejsonbalance-1002f-​common-issues-problemsolution401-unauthorizedcheck-your-api-key-id-and-private-key-file-pathsignature-errorensure-timestamp-is-in-milliseconds-not-secondspath-not-foundpath-must-start-with-trade-apiv2signature-error-with-query-paramsstrip-query-parameters-before-signing-use-pathsplit0-​next-steps-now-you-can-make-authenticated-requests-try-these-endpoints-trade-apiv2portfoliopositions-get-your-positions-trade-apiv2portfolioorders-view-your-orders-trade-apiv2markets-browse-available-markets-for-more-details-see-the-complete-order-lifecycle-guide-or-explore-the-api-reference)
        - [Page Not Found404Page Not FoundWe couldn't find the page. Maybe you were looking for one of these pages below?Quick Start: Create your first orderGet OrdersQuick Start: Authenticated RequestsPage Not Found](#page-not-found404page-not-foundwe-couldnt-find-the-page-maybe-you-were-looking-for-one-of-these-pages-belowquick-start-create-your-first-orderget-ordersquick-start-authenticated-requestspage-not-found)
        - [Test In The Demo Environment - API DocumentationTest In The Demo EnvironmentFor testing purposes, Kalshi offers a demo environment with mock funds. You can access the Demo environment at https://demo.kalshi.co/. For safety, credentials are not shared between this environment and production. To set up a Kalshi Demo account, follow this step-by-step tutorial. Demo’s API root is https://demo-api.kalshi.co/trade-api/v2.](#test-in-the-demo-environment-api-documentationtest-in-the-demo-environmentfor-testing-purposes-kalshi-offers-a-demo-environment-with-mock-funds-you-can-access-the-demo-environment-at-httpsdemokalshico-for-safety-credentials-are-not-shared-between-this-environment-and-production-to-set-up-a-kalshi-demo-account-follow-this-step-by-step-tutorial-demo’s-api-root-is-httpsdemo-apikalshicotrade-apiv2)
        - [Quick Start: Create your first order - API DocumentationQuick Start: Create your first orderThis guide will walk you through the complete lifecycle of placing and managing orders on Kalshi. ​Prerequisites Before you begin, you’ll need: A Kalshi account with API access configured Python with the requests and cryptography libraries installed Your authentication functions set up (see our authentication guide) This guide assumes you have the authentication code from our authentication guide, including the get() function for making authenticated requests. ​Step 1: Find an Open Market First, let’s find an open market to trade on. Copy# Get the first open market (no auth required for public market data) response = requests.get('https://demo-api.kalshi.co/trade-api/v2/markets?limit=1&status=open') market = response.json()['markets'][0] print(f"Selected market: {market['ticker']}") print(f"Title: {market['title']}") ​Step 2: Place a Buy Order Now let’s place an order to buy 1 YES contract for 1 cent (limit order). We’ll use a client_order_id to deduplicate orders - this allows you to identify duplicate orders before receiving the server-generated order_id in the response. Copyimport uuid def post(private_key, api_key_id, path, data, base_url=BASE_URL): """Make an authenticated POST request to the Kalshi API.""" timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) signature = create_signature(private_key, timestamp, "POST", path) headers = { 'KALSHI-ACCESS-KEY': api_key_id, 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp, 'Content-Type': 'application/json' } return requests.post(base_url + path, headers=headers, json=data) # Place a buy order for 1 YES contract at 1 cent order_data = { "ticker": market['ticker'], "action": "buy", "side": "yes", "count": 1, "type": "limit", "yes_price": 1, "client_order_id": str(uuid.uuid4()) # Unique ID for deduplication } response = post(private_key, API_KEY_ID, '/trade-api/v2/portfolio/orders', order_data) if response.status_code == 201: order = response.json()['order'] print(f"Order placed successfully!") print(f"Order ID: {order['order_id']}") print(f"Client Order ID: {order_data['client_order_id']}") print(f"Status: {order['status']}") else: print(f"Error: {response.status_code} - {response.text}") ​Complete Example Script Here’s a complete script that creates your first order: Copyimport requests import uuid # Assumes you have the authentication code from the prerequisites # Add POST function to your existing auth code def post(private_key, api_key_id, path, data, base_url=BASE_URL): """Make an authenticated POST request to the Kalshi API.""" timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) signature = create_signature(private_key, timestamp, "POST", path) headers = { 'KALSHI-ACCESS-KEY': api_key_id, 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp, 'Content-Type': 'application/json' } return requests.post(base_url + path, headers=headers, json=data) # Step 1: Find an open market print("Finding an open market...") response = requests.get('https://demo-api.kalshi.co/trade-api/v2/markets?limit=1&status=open') market = response.json()['markets'][0] print(f"Selected: {market['ticker']} - {market['title']}") # Step 2: Place a buy order print("\nPlacing order...") client_order_id = str(uuid.uuid4()) order_data = { "ticker": market['ticker'], "action": "buy", "side": "yes", "count": 1, "type": "limit", "yes_price": 1, "client_order_id": client_order_id } response = post(private_key, API_KEY_ID, '/trade-api/v2/portfolio/orders', order_data) if response.status_code == 201: order = response.json()['order'] print(f"Order placed successfully!") print(f"Order ID: {order['order_id']}") print(f"Client Order ID: {client_order_id}") print(f"Status: {order['status']}") else: print(f"Error: {response.status_code} - {response.text}") ​Important Notes ​Client Order ID The client_order_id field is crucial for order deduplication: Generate a unique ID (like UUID4) for each order before submission If network issues occur, you can resubmit with the same client_order_id The API will reject duplicate submissions, preventing accidental double orders Store this ID locally to track orders before receiving the server’s order_id ​Error Handling Common errors and how to handle them: 401 Unauthorized: Check your API keys and signature generation 400 Bad Request: Verify your order parameters (price must be 1-99 cents) 409 Conflict: Order with this client_order_id already exists 429 Too Many Requests: You’ve hit the rate limit - slow down your requests ​Next Steps Now that you’ve created your first order, you can: Check order status using the /portfolio/orders/{order_id} endpoint List all your orders with /portfolio/orders Amend your order price or quantity using PUT /portfolio/orders/{order_id} Cancel orders using DELETE /portfolio/orders/{order_id} Implement WebSocket connections for real-time updates Build automated trading strategies For more information, check out: API Reference Documentation Python Starter Code Kalshi Discord Community](#quick-start-create-your-first-order-api-documentationquick-start-create-your-first-orderthis-guide-will-walk-you-through-the-complete-lifecycle-of-placing-and-managing-orders-on-kalshi-​prerequisites-before-you-begin-you’ll-need-a-kalshi-account-with-api-access-configured-python-with-the-requests-and-cryptography-libraries-installed-your-authentication-functions-set-up-see-our-authentication-guide-this-guide-assumes-you-have-the-authentication-code-from-our-authentication-guide-including-the-get-function-for-making-authenticated-requests-​step-1-find-an-open-market-first-let’s-find-an-open-market-to-trade-on-copy-get-the-first-open-market-no-auth-required-for-public-market-data-response-requestsgethttpsdemo-apikalshicotrade-apiv2marketslimit1statusopen-market-responsejsonmarkets0-printfselected-market-marketticker-printftitle-markettitle-​step-2-place-a-buy-order-now-let’s-place-an-order-to-buy-1-yes-contract-for-1-cent-limit-order-we’ll-use-a-client_order_id-to-deduplicate-orders-this-allows-you-to-identify-duplicate-orders-before-receiving-the-server-generated-order_id-in-the-response-copyimport-uuid-def-postprivate_key-api_key_id-path-data-base_urlbase_url-make-an-authenticated-post-request-to-the-kalshi-api-timestamp-strintdatetimedatetimenowtimestamp-1000-signature-create_signatureprivate_key-timestamp-post-path-headers-kalshi-access-key-api_key_id-kalshi-access-signature-signature-kalshi-access-timestamp-timestamp-content-type-applicationjson-return-requestspostbase_url-path-headersheaders-jsondata-place-a-buy-order-for-1-yes-contract-at-1-cent-order_data-ticker-marketticker-action-buy-side-yes-count-1-type-limit-yes_price-1-client_order_id-struuiduuid4-unique-id-for-deduplication-response-postprivate_key-api_key_id-trade-apiv2portfolioorders-order_data-if-responsestatus_code-201-order-responsejsonorder-printforder-placed-successfully-printforder-id-orderorder_id-printfclient-order-id-order_dataclient_order_id-printfstatus-orderstatus-else-printferror-responsestatus_code-responsetext-​complete-example-script-here’s-a-complete-script-that-creates-your-first-order-copyimport-requests-import-uuid-assumes-you-have-the-authentication-code-from-the-prerequisites-add-post-function-to-your-existing-auth-code-def-postprivate_key-api_key_id-path-data-base_urlbase_url-make-an-authenticated-post-request-to-the-kalshi-api-timestamp-strintdatetimedatetimenowtimestamp-1000-signature-create_signatureprivate_key-timestamp-post-path-headers-kalshi-access-key-api_key_id-kalshi-access-signature-signature-kalshi-access-timestamp-timestamp-content-type-applicationjson-return-requestspostbase_url-path-headersheaders-jsondata-step-1-find-an-open-market-printfinding-an-open-market-response-requestsgethttpsdemo-apikalshicotrade-apiv2marketslimit1statusopen-market-responsejsonmarkets0-printfselected-marketticker-markettitle-step-2-place-a-buy-order-printnplacing-order-client_order_id-struuiduuid4-order_data-ticker-marketticker-action-buy-side-yes-count-1-type-limit-yes_price-1-client_order_id-client_order_id-response-postprivate_key-api_key_id-trade-apiv2portfolioorders-order_data-if-responsestatus_code-201-order-responsejsonorder-printforder-placed-successfully-printforder-id-orderorder_id-printfclient-order-id-client_order_id-printfstatus-orderstatus-else-printferror-responsestatus_code-responsetext-​important-notes-​client-order-id-the-client_order_id-field-is-crucial-for-order-deduplication-generate-a-unique-id-like-uuid4-for-each-order-before-submission-if-network-issues-occur-you-can-resubmit-with-the-same-client_order_id-the-api-will-reject-duplicate-submissions-preventing-accidental-double-orders-store-this-id-locally-to-track-orders-before-receiving-the-server’s-order_id-​error-handling-common-errors-and-how-to-handle-them-401-unauthorized-check-your-api-keys-and-signature-generation-400-bad-request-verify-your-order-parameters-price-must-be-1-99-cents-409-conflict-order-with-this-client_order_id-already-exists-429-too-many-requests-you’ve-hit-the-rate-limit-slow-down-your-requests-​next-steps-now-that-you’ve-created-your-first-order-you-can-check-order-status-using-the-portfolioordersorder_id-endpoint-list-all-your-orders-with-portfolioorders-amend-your-order-price-or-quantity-using-put-portfolioordersorder_id-cancel-orders-using-delete-portfolioordersorder_id-implement-websocket-connections-for-real-time-updates-build-automated-trading-strategies-for-more-information-check-out-api-reference-documentation-python-starter-code-kalshi-discord-community)
          - [API Keys - API DocumentationAPI KeysThis process is the same for the demo or production environment. ​Generating an API Key ​Access the Account Settings Page: Log in to your account and navigate to the “Account Settings” page. You can typically find this option by clicking on your profile picture or account icon in the top-right corner of the application. ​Generate a New API Key In the “Profile Settings” page https://kalshi.com/account/profile, locate the “API Keys” section. Click on the “Create New API Key” button. This action will generate a new API key in the RSA_PRIVATE_KEY format. ​Store Your API Key and Key ID: After generating the key, you will be presented with: • Private Key: This is your secret key in RSA_PRIVATE_KEY format. • Key ID: This is a unique identifier associated with your private key. Important: For security reasons, the private key will not be stored by our service, and you will not be able to retrieve it again once this page is closed. Please make sure to securely copy and save the private key immediately. The key will also be downloaded as txt file with the name provided. ​Using a API Key Each request to Kalshi trading api will need to be signed with the private key generated above. The following header values will need to be provided with each request: KALSHI-ACCESS-KEY- the Key ID KALSHI-ACCESS-TIMESTAMP - the request timestamp in ms KALSHI-ACCESS-SIGNATURE- request hash signed with private key The above signature is generated by signing a concatenation of the timestamp, the HTTP method and the path. Important: When signing requests, use the path without query parameters. For example, if your request is to /trade-api/v2/portfolio/orders?limit=5, sign only /trade-api/v2/portfolio/orders (strip the ? and everything after it). Sample code for generating the required headers is below (alternatively use our example code here): ​Python Load the private key stored in a file Copyfrom cryptography.hazmat.primitives import serialization from cryptography.hazmat.backends import default_backend def load_private_key_from_file(file_path): with open(file_path, "rb") as key_file: private_key = serialization.load_pem_private_key( key_file.read(), password=None, # or provide a password if your key is encrypted backend=default_backend() ) return private_key Sign text with private key Copyimport base64 from cryptography.hazmat.primitives import hashes from cryptography.hazmat.primitives.asymmetric import padding, rsa from cryptography.exceptions import InvalidSignature def sign_pss_text(private_key: rsa.RSAPrivateKey, text: str) -> str: message = text.encode('utf-8') try: signature = private_key.sign( message, padding.PSS( mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH ), hashes.SHA256() ) return base64.b64encode(signature).decode('utf-8') except InvalidSignature as e: raise ValueError("RSA sign PSS failed") from e Send a request to Kalshi API with signed header Copyimport requests import datetime current_time = datetime.datetime.now() timestamp = current_time.timestamp() current_time_milliseconds = int(timestamp * 1000) timestampt_str = str(current_time_milliseconds) private_key = load_private_key_from_file('kalshi-key-2.key') method = "GET" base_url = 'https://demo-api.kalshi.co' path='/trade-api/v2/portfolio/balance' # Strip query parameters from path before signing path_without_query = path.split('?')[0] msg_string = timestampt_str + method + path_without_query sig = sign_pss_text(private_key, msg_string) headers = { 'KALSHI-ACCESS-KEY': 'a952bcbe-ec3b-4b5b-b8f9-11dae589608c', 'KALSHI-ACCESS-SIGNATURE': sig, 'KALSHI-ACCESS-TIMESTAMP': timestampt_str } response = requests.get(base_url + path, headers=headers) print(response.text) ​Javascript Load the private key stored in a file Copyconst fs = require('fs'); const path = require('path'); function loadPrivateKeyFromFile(filePath) { const absolutePath = path.resolve(filePath); const privateKeyPem = fs.readFileSync(absolutePath, 'utf8'); return privateKeyPem; } Sign text with private key Copyconst crypto = require('crypto'); function signPssText(privateKeyPem, text) { const sign = crypto.createSign('RSA-SHA256'); sign.update(text); sign.end(); const signature = sign.sign({ key: privateKeyPem, padding: crypto.constants.RSA_PKCS1_PSS_PADDING, saltLength: crypto.constants.RSA_PSS_SALTLEN_DIGEST, }); return signature.toString('base64'); } Send a request to Kalshi API with signed header Copyconst axios = require('axios'); const currentTimeMilliseconds = Date.now(); const timestampStr = currentTimeMilliseconds.toString(); const privateKeyPem = loadPrivateKeyFromFile('path/to/your/private-key.pem'); const method = "GET"; const baseUrl = 'https://demo-api.kalshi.co'; const path = '/trade-api/v2/portfolio/balance'; // Strip query parameters from path before signing const pathWithoutQuery = path.split('?')[0]; const msgString = timestampStr + method + pathWithoutQuery; const sig = signPssText(privateKeyPem, msgString); const headers = { 'KALSHI-ACCESS-KEY': 'your-api-key-id', 'KALSHI-ACCESS-SIGNATURE': sig, 'KALSHI-ACCESS-TIMESTAMP': timestampStr }; axios.get(baseUrl + path, { headers }) .then(response => { console.log(response.data); }) .catch(error => { console.error('Error:', error); });](#api-keys-api-documentationapi-keysthis-process-is-the-same-for-the-demo-or-production-environment-​generating-an-api-key-​access-the-account-settings-page-log-in-to-your-account-and-navigate-to-the-“account-settings”-page-you-can-typically-find-this-option-by-clicking-on-your-profile-picture-or-account-icon-in-the-top-right-corner-of-the-application-​generate-a-new-api-key-in-the-“profile-settings”-page-httpskalshicomaccountprofile-locate-the-“api-keys”-section-click-on-the-“create-new-api-key”-button-this-action-will-generate-a-new-api-key-in-the-rsa_private_key-format-​store-your-api-key-and-key-id-after-generating-the-key-you-will-be-presented-with-•-private-key-this-is-your-secret-key-in-rsa_private_key-format-•-key-id-this-is-a-unique-identifier-associated-with-your-private-key-important-for-security-reasons-the-private-key-will-not-be-stored-by-our-service-and-you-will-not-be-able-to-retrieve-it-again-once-this-page-is-closed-please-make-sure-to-securely-copy-and-save-the-private-key-immediately-the-key-will-also-be-downloaded-as-txt-file-with-the-name-provided-​using-a-api-key-each-request-to-kalshi-trading-api-will-need-to-be-signed-with-the-private-key-generated-above-the-following-header-values-will-need-to-be-provided-with-each-request-kalshi-access-key-the-key-id-kalshi-access-timestamp-the-request-timestamp-in-ms-kalshi-access-signature-request-hash-signed-with-private-key-the-above-signature-is-generated-by-signing-a-concatenation-of-the-timestamp-the-http-method-and-the-path-important-when-signing-requests-use-the-path-without-query-parameters-for-example-if-your-request-is-to-trade-apiv2portfolioorderslimit5-sign-only-trade-apiv2portfolioorders-strip-the-and-everything-after-it-sample-code-for-generating-the-required-headers-is-below-alternatively-use-our-example-code-here-​python-load-the-private-key-stored-in-a-file-copyfrom-cryptographyhazmatprimitives-import-serialization-from-cryptographyhazmatbackends-import-default_backend-def-load_private_key_from_filefile_path-with-openfile_path-rb-as-key_file-private_key-serializationload_pem_private_key-key_fileread-passwordnone-or-provide-a-password-if-your-key-is-encrypted-backenddefault_backend-return-private_key-sign-text-with-private-key-copyimport-base64-from-cryptographyhazmatprimitives-import-hashes-from-cryptographyhazmatprimitivesasymmetric-import-padding-rsa-from-cryptographyexceptions-import-invalidsignature-def-sign_pss_textprivate_key-rsarsaprivatekey-text-str-str-message-textencodeutf-8-try-signature-private_keysign-message-paddingpss-mgfpaddingmgf1hashessha256-salt_lengthpaddingpssdigest_length-hashessha256-return-base64b64encodesignaturedecodeutf-8-except-invalidsignature-as-e-raise-valueerrorrsa-sign-pss-failed-from-e-send-a-request-to-kalshi-api-with-signed-header-copyimport-requests-import-datetime-current_time-datetimedatetimenow-timestamp-current_timetimestamp-current_time_milliseconds-inttimestamp-1000-timestampt_str-strcurrent_time_milliseconds-private_key-load_private_key_from_filekalshi-key-2key-method-get-base_url-httpsdemo-apikalshico-pathtrade-apiv2portfoliobalance-strip-query-parameters-from-path-before-signing-path_without_query-pathsplit0-msg_string-timestampt_str-method-path_without_query-sig-sign_pss_textprivate_key-msg_string-headers-kalshi-access-key-a952bcbe-ec3b-4b5b-b8f9-11dae589608c-kalshi-access-signature-sig-kalshi-access-timestamp-timestampt_str-response-requestsgetbase_url-path-headersheaders-printresponsetext-​javascript-load-the-private-key-stored-in-a-file-copyconst-fs-requirefs-const-path-requirepath-function-loadprivatekeyfromfilefilepath-const-absolutepath-pathresolvefilepath-const-privatekeypem-fsreadfilesyncabsolutepath-utf8-return-privatekeypem-sign-text-with-private-key-copyconst-crypto-requirecrypto-function-signpsstextprivatekeypem-text-const-sign-cryptocreatesignrsa-sha256-signupdatetext-signend-const-signature-signsign-key-privatekeypem-padding-cryptoconstantsrsa_pkcs1_pss_padding-saltlength-cryptoconstantsrsa_pss_saltlen_digest-return-signaturetostringbase64-send-a-request-to-kalshi-api-with-signed-header-copyconst-axios-requireaxios-const-currenttimemilliseconds-datenow-const-timestampstr-currenttimemillisecondstostring-const-privatekeypem-loadprivatekeyfromfilepathtoyourprivate-keypem-const-method-get-const-baseurl-httpsdemo-apikalshico-const-path-trade-apiv2portfoliobalance-strip-query-parameters-from-path-before-signing-const-pathwithoutquery-pathsplit0-const-msgstring-timestampstr-method-pathwithoutquery-const-sig-signpsstextprivatekeypem-msgstring-const-headers-kalshi-access-key-your-api-key-id-kalshi-access-signature-sig-kalshi-access-timestamp-timestampstr-axiosgetbaseurl-path-headers-thenresponse-consolelogresponsedata-catcherror-consoleerrorerror-error)
          - [Orderbook Responses - API DocumentationOrderbook Responses​Getting Orderbook Data The Get Market Orderbook endpoint returns the current state of bids for a specific market. ​Request Format CopyGET /markets/{ticker}/orderbook No authentication is required for this endpoint. ​Example Request PythonJavaScriptcURLCopyimport requests # Get orderbook for a specific market market_ticker = "KXHIGHNY-24JAN01-T60" url = f"https://api.elections.kalshi.com/trade-api/v2/markets/{market_ticker}/orderbook" response = requests.get(url) orderbook_data = response.json() ​Response Structure The orderbook response contains two arrays of bids - one for YES positions and one for NO positions. Each bid is represented as a two-element array: [price, quantity]. ​Example Response Copy{ "orderbook": { "yes": [ [1, 200], // 200 contracts bid at 1¢ [15, 100], // 100 contracts bid at 15¢ [20, 50], // 50 contracts bid at 20¢ [25, 20], // 20 contracts bid at 25¢ [30, 11], // 11 contracts bid at 30¢ [31, 10], // 10 contracts bid at 31¢ [32, 10], // 10 contracts bid at 32¢ [33, 11], // 11 contracts bid at 33¢ [34, 9], // 9 contracts bid at 34¢ [35, 11], // 11 contracts bid at 35¢ [41, 10], // 10 contracts bid at 41¢ [42, 13] // 13 contracts bid at 42¢ ], "no": [ [1, 100], // 100 contracts bid at 1¢ [16, 3], // 3 contracts bid at 16¢ [25, 50], // 50 contracts bid at 25¢ [28, 19], // 19 contracts bid at 28¢ [36, 5], // 5 contracts bid at 36¢ [37, 50], // 50 contracts bid at 37¢ [38, 300], // 300 contracts bid at 38¢ [44, 29], // 29 contracts bid at 44¢ [45, 20], // 20 contracts bid at 45¢ [56, 17] // 17 contracts bid at 56¢ ] } ​Understanding the Arrays First element: Price in cents (1-99) Second element: Number of contracts available at that price Arrays are sorted by price in ascending order The highest bid (best bid) is the last element in each array ​Why Only Bids? Important: Kalshi’s orderbook only returns bids, not asks. This is because in binary prediction markets, there’s a reciprocal relationship between YES and NO positions. In binary prediction markets, every position has a complementary opposite: A YES BID at price X is equivalent to a NO ASK at price (100 - X) A NO BID at price Y is equivalent to a YES ASK at price (100 - Y) ​The Reciprocal Relationship Since binary markets must sum to 100¢, these relationships always hold: ActionEquivalent ToWhyYES BID at 60¢NO ASK at 40¢Willing to pay 60¢ for YES = Willing to receive 40¢ to take NONO BID at 30¢YES ASK at 70¢Willing to pay 30¢ for NO = Willing to receive 70¢ to take YES This reciprocal nature means that by showing only bids, the orderbook provides complete market information while avoiding redundancy. ​Calculating Spreads To find the bid-ask spread for a market: YES spread: Best YES bid: Highest price in the yes array Best YES ask: 100 - (Highest price in the no array) Spread = Best YES ask - Best YES bid NO spread: Best NO bid: Highest price in the no array Best NO ask: 100 - (Highest price in the yes array) Spread = Best NO ask - Best NO bid ​Example Calculation Copy# Using the example orderbook above best_yes_bid = 42 # Highest YES bid (last in array) best_yes_ask = 100 - 56 # 100 - highest NO bid = 44 spread = best_yes_ask - best_yes_bid # 44 - 42 = 2 # The spread is 2¢ # You can buy YES at 44¢ (implied ask) and sell at 42¢ (bid) ​Working with Orderbook Data ​Display Best Prices PythonJavaScriptCopydef display_best_prices(orderbook_data): """Display the best bid prices and implied asks""" orderbook = orderbook_data['orderbook'] # Best bids (if any exist) if orderbook['yes']: best_yes_bid = orderbook['yes'][-1][0] # Last element is highest print(f"Best YES Bid: {best_yes_bid}¢") if orderbook['no']: best_no_bid = orderbook['no'][-1][0] # Last element is highest best_yes_ask = 100 - best_no_bid print(f"Best YES Ask: {best_yes_ask}¢ (implied from NO bid)") print() if orderbook['no']: best_no_bid = orderbook['no'][-1][0] # Last element is highest print(f"Best NO Bid: {best_no_bid}¢") if orderbook['yes']: best_yes_bid = orderbook['yes'][-1][0] # Last element is highest best_no_ask = 100 - best_yes_bid print(f"Best NO Ask: {best_no_ask}¢ (implied from YES bid)") ​Calculate Market Depth Copydef calculate_depth(orderbook_data, depth_cents=5): """Calculate total volume within X cents of best bid""" orderbook = orderbook_data['orderbook'] yes_depth = 0 no_depth = 0 # YES side depth (iterate backwards from best bid) if orderbook['yes']: best_yes = orderbook['yes'][-1][0] # Last element is highest for price, quantity in reversed(orderbook['yes']): if best_yes - price <= depth_cents: yes_depth += quantity else: break # NO side depth (iterate backwards from best bid) if orderbook['no']: best_no = orderbook['no'][-1][0] # Last element is highest for price, quantity in reversed(orderbook['no']): if best_no - price <= depth_cents: no_depth += quantity else: break return {"yes_depth": yes_depth, "no_depth": no_depth} ​Next Steps Learn about making authenticated requests to place orders Explore WebSocket connections for real-time orderbook updates Read about market mechanics on the Kalshi website](#orderbook-responses-api-documentationorderbook-responses​getting-orderbook-data-the-get-market-orderbook-endpoint-returns-the-current-state-of-bids-for-a-specific-market-​request-format-copyget-marketstickerorderbook-no-authentication-is-required-for-this-endpoint-​example-request-pythonjavascriptcurlcopyimport-requests-get-orderbook-for-a-specific-market-market_ticker-kxhighny-24jan01-t60-url-fhttpsapielectionskalshicomtrade-apiv2marketsmarket_tickerorderbook-response-requestsgeturl-orderbook_data-responsejson-​response-structure-the-orderbook-response-contains-two-arrays-of-bids-one-for-yes-positions-and-one-for-no-positions-each-bid-is-represented-as-a-two-element-array-price-quantity-​example-response-copy-orderbook-yes-1-200-200-contracts-bid-at-1¢-15-100-100-contracts-bid-at-15¢-20-50-50-contracts-bid-at-20¢-25-20-20-contracts-bid-at-25¢-30-11-11-contracts-bid-at-30¢-31-10-10-contracts-bid-at-31¢-32-10-10-contracts-bid-at-32¢-33-11-11-contracts-bid-at-33¢-34-9-9-contracts-bid-at-34¢-35-11-11-contracts-bid-at-35¢-41-10-10-contracts-bid-at-41¢-42-13-13-contracts-bid-at-42¢-no-1-100-100-contracts-bid-at-1¢-16-3-3-contracts-bid-at-16¢-25-50-50-contracts-bid-at-25¢-28-19-19-contracts-bid-at-28¢-36-5-5-contracts-bid-at-36¢-37-50-50-contracts-bid-at-37¢-38-300-300-contracts-bid-at-38¢-44-29-29-contracts-bid-at-44¢-45-20-20-contracts-bid-at-45¢-56-17-17-contracts-bid-at-56¢-​understanding-the-arrays-first-element-price-in-cents-1-99-second-element-number-of-contracts-available-at-that-price-arrays-are-sorted-by-price-in-ascending-order-the-highest-bid-best-bid-is-the-last-element-in-each-array-​why-only-bids-important-kalshi’s-orderbook-only-returns-bids-not-asks-this-is-because-in-binary-prediction-markets-there’s-a-reciprocal-relationship-between-yes-and-no-positions-in-binary-prediction-markets-every-position-has-a-complementary-opposite-a-yes-bid-at-price-x-is-equivalent-to-a-no-ask-at-price-100-x-a-no-bid-at-price-y-is-equivalent-to-a-yes-ask-at-price-100-y-​the-reciprocal-relationship-since-binary-markets-must-sum-to-100¢-these-relationships-always-hold-actionequivalent-towhyyes-bid-at-60¢no-ask-at-40¢willing-to-pay-60¢-for-yes-willing-to-receive-40¢-to-take-nono-bid-at-30¢yes-ask-at-70¢willing-to-pay-30¢-for-no-willing-to-receive-70¢-to-take-yes-this-reciprocal-nature-means-that-by-showing-only-bids-the-orderbook-provides-complete-market-information-while-avoiding-redundancy-​calculating-spreads-to-find-the-bid-ask-spread-for-a-market-yes-spread-best-yes-bid-highest-price-in-the-yes-array-best-yes-ask-100-highest-price-in-the-no-array-spread-best-yes-ask-best-yes-bid-no-spread-best-no-bid-highest-price-in-the-no-array-best-no-ask-100-highest-price-in-the-yes-array-spread-best-no-ask-best-no-bid-​example-calculation-copy-using-the-example-orderbook-above-best_yes_bid-42-highest-yes-bid-last-in-array-best_yes_ask-100-56-100-highest-no-bid-44-spread-best_yes_ask-best_yes_bid-44-42-2-the-spread-is-2¢-you-can-buy-yes-at-44¢-implied-ask-and-sell-at-42¢-bid-​working-with-orderbook-data-​display-best-prices-pythonjavascriptcopydef-display_best_pricesorderbook_data-display-the-best-bid-prices-and-implied-asks-orderbook-orderbook_dataorderbook-best-bids-if-any-exist-if-orderbookyes-best_yes_bid-orderbookyes-10-last-element-is-highest-printfbest-yes-bid-best_yes_bid¢-if-orderbookno-best_no_bid-orderbookno-10-last-element-is-highest-best_yes_ask-100-best_no_bid-printfbest-yes-ask-best_yes_ask¢-implied-from-no-bid-print-if-orderbookno-best_no_bid-orderbookno-10-last-element-is-highest-printfbest-no-bid-best_no_bid¢-if-orderbookyes-best_yes_bid-orderbookyes-10-last-element-is-highest-best_no_ask-100-best_yes_bid-printfbest-no-ask-best_no_ask¢-implied-from-yes-bid-​calculate-market-depth-copydef-calculate_depthorderbook_data-depth_cents5-calculate-total-volume-within-x-cents-of-best-bid-orderbook-orderbook_dataorderbook-yes_depth-0-no_depth-0-yes-side-depth-iterate-backwards-from-best-bid-if-orderbookyes-best_yes-orderbookyes-10-last-element-is-highest-for-price-quantity-in-reversedorderbookyes-if-best_yes-price-depth_cents-yes_depth-quantity-else-break-no-side-depth-iterate-backwards-from-best-bid-if-orderbookno-best_no-orderbookno-10-last-element-is-highest-for-price-quantity-in-reversedorderbookno-if-best_no-price-depth_cents-no_depth-quantity-else-break-return-yes_depth-yes_depth-no_depth-no_depth-​next-steps-learn-about-making-authenticated-requests-to-place-orders-explore-websocket-connections-for-real-time-orderbook-updates-read-about-market-mechanics-on-the-kalshi-website)
          - [Quick Start: WebSockets - API DocumentationQuick Start: WebSockets​Overview Kalshi’s WebSocket API provides real-time updates for: Order book changes Trade executions Market status updates Fill notifications (authenticated connections only) ​Connection URL Connect to the WebSocket endpoint at: Copywss://api.elections.kalshi.com/trade-api/ws/v2 For the demo environment, use: Copywss://demo-api.kalshi.co/trade-api/ws/v2 ​Authentication WebSocket connections require authentication using the same API key signing mechanism as REST endpoints. For detailed information about API key generation and request signing, see our API Keys documentation. ​Required Headers When establishing the WebSocket connection, include these headers: CopyKALSHI-ACCESS-KEY: your_api_key_id KALSHI-ACCESS-SIGNATURE: request_signature KALSHI-ACCESS-TIMESTAMP: unix_timestamp_in_milliseconds ​Signing the WebSocket Request The signature for WebSocket connections follows the same pattern as REST API requests: Create the message to sign: Copytimestamp + "GET" + "/trade-api/ws/v2" Generate the signature using your private key (see API Keys documentation) Include the headers when opening the WebSocket connection ​Establishing a Connection To connect to the WebSocket API, you need to: Generate authentication headers (same as REST API) Create a WebSocket connection with those headers Handle the connection lifecycle Here’s how to establish an authenticated connection: Copyimport websockets import asyncio # WebSocket URL ws_url = "wss://demo-api.kalshi.co/trade-api/ws/v2" # Demo environment # Generate authentication headers (see API Keys documentation) auth_headers = { "KALSHI-ACCESS-KEY": "your_api_key_id", "KALSHI-ACCESS-SIGNATURE": "generated_signature", "KALSHI-ACCESS-TIMESTAMP": "timestamp_in_milliseconds" } # Connect with authentication async def connect(): async with websockets.connect(ws_url, additional_headers=auth_headers) as websocket: print("Connected to Kalshi WebSocket") # Connection is now established # You can start sending and receiving messages # Listen for messages async for message in websocket: print(f"Received: {message}") # Run the connection asyncio.run(connect()) ​Subscribing to Data Once connected, subscribe to channels by sending a subscription command: Copyimport json async def subscribe_to_ticker(websocket): """Subscribe to ticker updates""" subscription = { "id": 1, "cmd": "subscribe", "params": { "channels": ["ticker"] } await websocket.send(json.dumps(subscription)) async def subscribe_to_orderbook(websocket, market_tickers): """Subscribe to orderbook updates for specific markets""" subscription = { "id": 2, "cmd": "subscribe", "params": { "channels": ["orderbook_delta"], "market_tickers": market_tickers } await websocket.send(json.dumps(subscription)) ​Processing Messages Handle incoming messages based on their type: Copyasync def process_message(message): """Process incoming WebSocket messages""" data = json.loads(message) msg_type = data.get("type") if msg_type == "ticker": # Handle ticker update market = data["data"]["market_ticker"] bid = data["data"]["bid"] ask = data["data"]["ask"] print(f"{market}: Bid ${bid}, Ask ${ask}") elif msg_type == "orderbook_snapshot": # Handle full orderbook state print(f"Orderbook snapshot for {data['data']['market_ticker']}") elif msg_type == "orderbook_update": # Handle orderbook changes print(f"Orderbook update for {data['data']['market_ticker']}") # Note: client_order_id field is optional - present only when you caused this change if 'client_order_id' in data['data']: print(f" Your order {data['data']['client_order_id']} caused this change") elif msg_type == "error": error_code = data.get("msg", {}).get("code") error_msg = data.get("msg", {}).get("msg") print(f"Error {error_code}: {error_msg}") ​Connection Keep-Alive The Python websockets library automatically handles WebSocket ping/pong frames to keep connections alive. No manual heartbeat handling is required. Learn more about automatic keepalive in the websockets documentation.Other WebSocket libraries may require manual ping/pong implementation. ​Subscribing to Channels Once connected, subscribe to specific data channels: ​Subscribe to Ticker Updates To receive real-time ticker updates for all markets: Copyasync def subscribe_to_tickers(self): """Subscribe to ticker updates for all markets""" subscription_message = { "id": self.message_id, "cmd": "subscribe", "params": { "channels": ["ticker"] } await self.ws.send(json.dumps(subscription_message)) self.message_id += 1 ​Subscribe to Specific Markets To subscribe to orderbook or trade updates for specific markets: Copyasync def subscribe_to_markets(self, channels, market_tickers): """Subscribe to specific channels and markets""" subscription_message = { "id": self.message_id, "cmd": "subscribe", "params": { "channels": channels, "market_tickers": market_tickers } await self.ws.send(json.dumps(subscription_message)) self.message_id += 1 # Example usage: # Subscribe to orderbook updates await subscribe_to_markets(["orderbook"], ["KXFUT24-LSV", "KXHARRIS24-LSV"]) # Subscribe to trade feed await subscribe_to_markets(["trades"], ["KXFUT24-LSV"]) ​Connection Lifecycle Initial Connection: Establish WebSocket with authentication headers Subscribe: Send subscription commands for desired channels Receive Updates: Process incoming messages based on their type Handle Disconnects: Implement reconnection logic with exponential backoff ​Error Handling The server sends error messages in this format: Copy{ "id": 123, "type": "error", "msg": { "code": 6, "msg": "Params required" } ​WebSocket Error Codes CodeErrorDescription1Unable to process messageGeneral processing error2Params requiredMissing params object in command3Channels requiredMissing channels array in subscribe4Subscription IDs requiredMissing sids in unsubscribe5Unknown commandInvalid command name7Unknown subscription IDSubscription ID not found8Unknown channel nameInvalid channel in subscribe9Authentication requiredPrivate channel without auth10Channel errorChannel-specific error11Invalid parameterMalformed parameter value12Exactly one subscription ID requiredFor update_subscription13Unsupported actionInvalid action for update_subscription14Market ticker requiredMissing market specification15Action requiredMissing action in update_subscription16Market not foundInvalid market ticker17Internal errorServer-side processing error ​Best Practices Connection Management Implement automatic reconnection with exponential backoff Handle network interruptions gracefully Use the websockets library’s built-in keepalive Data Handling Process messages asynchronously to avoid blocking Implement proper error handling for malformed messages Cache initial orderbook state before applying updates Security Never expose your private key in client-side code Rotate API keys regularly Use secure key storage practices Performance Subscribe only to markets you need Implement message buffering for high-frequency updates Consider using connection pooling for multiple subscriptions ​Complete Example Here’s a complete, runnable example that connects to the WebSocket API and subscribes to orderbook updates: Copyimport asyncio import base64 import json import time import websockets from cryptography.hazmat.primitives import serialization, hashes from cryptography.hazmat.primitives.asymmetric import padding # Configuration KEY_ID = "your_api_key_id" PRIVATE_KEY_PATH = "path/to/private_key.pem" MARKET_TICKER = "KXHARRIS24-LSV" # Replace with any open market WS_URL = "wss://demo-api.kalshi.co/trade-api/ws/v2" def sign_pss_text(private_key, text: str) -> str: """Sign message using RSA-PSS""" message = text.encode('utf-8') signature = private_key.sign( message, padding.PSS( mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH ), hashes.SHA256() ) return base64.b64encode(signature).decode('utf-8') def create_headers(private_key, method: str, path: str) -> dict: """Create authentication headers""" timestamp = str(int(time.time() * 1000)) msg_string = timestamp + method + path.split('?')[0] signature = sign_pss_text(private_key, msg_string) return { "Content-Type": "application/json", "KALSHI-ACCESS-KEY": KEY_ID, "KALSHI-ACCESS-SIGNATURE": signature, "KALSHI-ACCESS-TIMESTAMP": timestamp, } async def orderbook_websocket(): """Connect to WebSocket and subscribe to orderbook""" # Load private key with open(PRIVATE_KEY_PATH, 'rb') as f: private_key = serialization.load_pem_private_key( f.read(), password=None ) # Create WebSocket headers ws_headers = create_headers(private_key, "GET", "/trade-api/ws/v2") async with websockets.connect(WS_URL, additional_headers=ws_headers) as websocket: print(f"Connected! Subscribing to orderbook for {MARKET_TICKER}") # Subscribe to orderbook subscribe_msg = { "id": 1, "cmd": "subscribe", "params": { "channels": ["orderbook_delta"], "market_ticker": MARKET_TICKER } await websocket.send(json.dumps(subscribe_msg)) # Process messages async for message in websocket: data = json.loads(message) msg_type = data.get("type") if msg_type == "subscribed": print(f"Subscribed: {data}") elif msg_type == "orderbook_snapshot": print(f"Orderbook snapshot: {data}") elif msg_type == "orderbook_delta": # The client_order_id field is optional - only present when you caused the change if 'client_order_id' in data.get('data', {}): print(f"Orderbook update (your order {data['data']['client_order_id']}): {data}") else: print(f"Orderbook update: {data}") elif msg_type == "error": print(f"Error: {data}") # Run the example if __name__ == "__main__": asyncio.run(orderbook_websocket()) This example: Establishes an authenticated WebSocket connection Subscribes to orderbook updates for the specified market Processes both the initial snapshot and incremental updates Displays orderbook changes in real-time To run this example: Replace KEY_ID with your API key ID Replace PRIVATE_KEY_PATH with the path to your private key file Replace MARKET_TICKER with any open market ticker Run with Python 3.7+ ​Next Steps Review the WebSocket API Reference for detailed message specifications Explore Market Data Quick Start for REST API integration Check out our Demo Environment for testing](#quick-start-websockets-api-documentationquick-start-websockets​overview-kalshi’s-websocket-api-provides-real-time-updates-for-order-book-changes-trade-executions-market-status-updates-fill-notifications-authenticated-connections-only-​connection-url-connect-to-the-websocket-endpoint-at-copywssapielectionskalshicomtrade-apiwsv2-for-the-demo-environment-use-copywssdemo-apikalshicotrade-apiwsv2-​authentication-websocket-connections-require-authentication-using-the-same-api-key-signing-mechanism-as-rest-endpoints-for-detailed-information-about-api-key-generation-and-request-signing-see-our-api-keys-documentation-​required-headers-when-establishing-the-websocket-connection-include-these-headers-copykalshi-access-key-your_api_key_id-kalshi-access-signature-request_signature-kalshi-access-timestamp-unix_timestamp_in_milliseconds-​signing-the-websocket-request-the-signature-for-websocket-connections-follows-the-same-pattern-as-rest-api-requests-create-the-message-to-sign-copytimestamp-get-trade-apiwsv2-generate-the-signature-using-your-private-key-see-api-keys-documentation-include-the-headers-when-opening-the-websocket-connection-​establishing-a-connection-to-connect-to-the-websocket-api-you-need-to-generate-authentication-headers-same-as-rest-api-create-a-websocket-connection-with-those-headers-handle-the-connection-lifecycle-here’s-how-to-establish-an-authenticated-connection-copyimport-websockets-import-asyncio-websocket-url-ws_url-wssdemo-apikalshicotrade-apiwsv2-demo-environment-generate-authentication-headers-see-api-keys-documentation-auth_headers-kalshi-access-key-your_api_key_id-kalshi-access-signature-generated_signature-kalshi-access-timestamp-timestamp_in_milliseconds-connect-with-authentication-async-def-connect-async-with-websocketsconnectws_url-additional_headersauth_headers-as-websocket-printconnected-to-kalshi-websocket-connection-is-now-established-you-can-start-sending-and-receiving-messages-listen-for-messages-async-for-message-in-websocket-printfreceived-message-run-the-connection-asynciorunconnect-​subscribing-to-data-once-connected-subscribe-to-channels-by-sending-a-subscription-command-copyimport-json-async-def-subscribe_to_tickerwebsocket-subscribe-to-ticker-updates-subscription-id-1-cmd-subscribe-params-channels-ticker-await-websocketsendjsondumpssubscription-async-def-subscribe_to_orderbookwebsocket-market_tickers-subscribe-to-orderbook-updates-for-specific-markets-subscription-id-2-cmd-subscribe-params-channels-orderbook_delta-market_tickers-market_tickers-await-websocketsendjsondumpssubscription-​processing-messages-handle-incoming-messages-based-on-their-type-copyasync-def-process_messagemessage-process-incoming-websocket-messages-data-jsonloadsmessage-msg_type-datagettype-if-msg_type-ticker-handle-ticker-update-market-datadatamarket_ticker-bid-datadatabid-ask-datadataask-printfmarket-bid-bid-ask-ask-elif-msg_type-orderbook_snapshot-handle-full-orderbook-state-printforderbook-snapshot-for-datadatamarket_ticker-elif-msg_type-orderbook_update-handle-orderbook-changes-printforderbook-update-for-datadatamarket_ticker-note-client_order_id-field-is-optional-present-only-when-you-caused-this-change-if-client_order_id-in-datadata-printf-your-order-datadataclient_order_id-caused-this-change-elif-msg_type-error-error_code-datagetmsg-getcode-error_msg-datagetmsg-getmsg-printferror-error_code-error_msg-​connection-keep-alive-the-python-websockets-library-automatically-handles-websocket-pingpong-frames-to-keep-connections-alive-no-manual-heartbeat-handling-is-required-learn-more-about-automatic-keepalive-in-the-websockets-documentationother-websocket-libraries-may-require-manual-pingpong-implementation-​subscribing-to-channels-once-connected-subscribe-to-specific-data-channels-​subscribe-to-ticker-updates-to-receive-real-time-ticker-updates-for-all-markets-copyasync-def-subscribe_to_tickersself-subscribe-to-ticker-updates-for-all-markets-subscription_message-id-selfmessage_id-cmd-subscribe-params-channels-ticker-await-selfwssendjsondumpssubscription_message-selfmessage_id-1-​subscribe-to-specific-markets-to-subscribe-to-orderbook-or-trade-updates-for-specific-markets-copyasync-def-subscribe_to_marketsself-channels-market_tickers-subscribe-to-specific-channels-and-markets-subscription_message-id-selfmessage_id-cmd-subscribe-params-channels-channels-market_tickers-market_tickers-await-selfwssendjsondumpssubscription_message-selfmessage_id-1-example-usage-subscribe-to-orderbook-updates-await-subscribe_to_marketsorderbook-kxfut24-lsv-kxharris24-lsv-subscribe-to-trade-feed-await-subscribe_to_marketstrades-kxfut24-lsv-​connection-lifecycle-initial-connection-establish-websocket-with-authentication-headers-subscribe-send-subscription-commands-for-desired-channels-receive-updates-process-incoming-messages-based-on-their-type-handle-disconnects-implement-reconnection-logic-with-exponential-backoff-​error-handling-the-server-sends-error-messages-in-this-format-copy-id-123-type-error-msg-code-6-msg-params-required-​websocket-error-codes-codeerrordescription1unable-to-process-messagegeneral-processing-error2params-requiredmissing-params-object-in-command3channels-requiredmissing-channels-array-in-subscribe4subscription-ids-requiredmissing-sids-in-unsubscribe5unknown-commandinvalid-command-name7unknown-subscription-idsubscription-id-not-found8unknown-channel-nameinvalid-channel-in-subscribe9authentication-requiredprivate-channel-without-auth10channel-errorchannel-specific-error11invalid-parametermalformed-parameter-value12exactly-one-subscription-id-requiredfor-update_subscription13unsupported-actioninvalid-action-for-update_subscription14market-ticker-requiredmissing-market-specification15action-requiredmissing-action-in-update_subscription16market-not-foundinvalid-market-ticker17internal-errorserver-side-processing-error-​best-practices-connection-management-implement-automatic-reconnection-with-exponential-backoff-handle-network-interruptions-gracefully-use-the-websockets-library’s-built-in-keepalive-data-handling-process-messages-asynchronously-to-avoid-blocking-implement-proper-error-handling-for-malformed-messages-cache-initial-orderbook-state-before-applying-updates-security-never-expose-your-private-key-in-client-side-code-rotate-api-keys-regularly-use-secure-key-storage-practices-performance-subscribe-only-to-markets-you-need-implement-message-buffering-for-high-frequency-updates-consider-using-connection-pooling-for-multiple-subscriptions-​complete-example-here’s-a-complete-runnable-example-that-connects-to-the-websocket-api-and-subscribes-to-orderbook-updates-copyimport-asyncio-import-base64-import-json-import-time-import-websockets-from-cryptographyhazmatprimitives-import-serialization-hashes-from-cryptographyhazmatprimitivesasymmetric-import-padding-configuration-key_id-your_api_key_id-private_key_path-pathtoprivate_keypem-market_ticker-kxharris24-lsv-replace-with-any-open-market-ws_url-wssdemo-apikalshicotrade-apiwsv2-def-sign_pss_textprivate_key-text-str-str-sign-message-using-rsa-pss-message-textencodeutf-8-signature-private_keysign-message-paddingpss-mgfpaddingmgf1hashessha256-salt_lengthpaddingpssdigest_length-hashessha256-return-base64b64encodesignaturedecodeutf-8-def-create_headersprivate_key-method-str-path-str-dict-create-authentication-headers-timestamp-strinttimetime-1000-msg_string-timestamp-method-pathsplit0-signature-sign_pss_textprivate_key-msg_string-return-content-type-applicationjson-kalshi-access-key-key_id-kalshi-access-signature-signature-kalshi-access-timestamp-timestamp-async-def-orderbook_websocket-connect-to-websocket-and-subscribe-to-orderbook-load-private-key-with-openprivate_key_path-rb-as-f-private_key-serializationload_pem_private_key-fread-passwordnone-create-websocket-headers-ws_headers-create_headersprivate_key-get-trade-apiwsv2-async-with-websocketsconnectws_url-additional_headersws_headers-as-websocket-printfconnected-subscribing-to-orderbook-for-market_ticker-subscribe-to-orderbook-subscribe_msg-id-1-cmd-subscribe-params-channels-orderbook_delta-market_ticker-market_ticker-await-websocketsendjsondumpssubscribe_msg-process-messages-async-for-message-in-websocket-data-jsonloadsmessage-msg_type-datagettype-if-msg_type-subscribed-printfsubscribed-data-elif-msg_type-orderbook_snapshot-printforderbook-snapshot-data-elif-msg_type-orderbook_delta-the-client_order_id-field-is-optional-only-present-when-you-caused-the-change-if-client_order_id-in-datagetdata-printforderbook-update-your-order-datadataclient_order_id-data-else-printforderbook-update-data-elif-msg_type-error-printferror-data-run-the-example-if-__name__-__main__-asynciorunorderbook_websocket-this-example-establishes-an-authenticated-websocket-connection-subscribes-to-orderbook-updates-for-the-specified-market-processes-both-the-initial-snapshot-and-incremental-updates-displays-orderbook-changes-in-real-time-to-run-this-example-replace-key_id-with-your-api-key-id-replace-private_key_path-with-the-path-to-your-private-key-file-replace-market_ticker-with-any-open-market-ticker-run-with-python-37-​next-steps-review-the-websocket-api-reference-for-detailed-message-specifications-explore-market-data-quick-start-for-rest-api-integration-check-out-our-demo-environment-for-testing)
          - [Rate Limits and Tiers - API DocumentationRate Limits and Tiers​Access tiers TierReadWriteBasic20 per second10 per secondAdvanced30 per second30 per secondPremier100 per second100 per secondPrime400 per second400 per second Qualification for tiers: Basic: Completing signup Advanced: Completing https://kalshi.typeform.com/advanced-api Premier: 3.75% of exchange traded volume in a given month Prime: 7.5% of exchange traded volume in a given month In addition to the volume targets, technical competency is a requirement for Premier/Prime access. Before providing access to the Premier/Prime tiers, the Exchange will establish that the trader/trading entity has the following requirements met: Knowledge of common security practices for API usage Proficiency in setting up monitoring for API usage, and ability to monitor API usage in near real-time Understanding and implementation of rate limiting and throttling mechanisms imposed by the API, and the ability to self-limit load Awareness of legal and compliance aspects related to API usage Only the following APIs fall under the write limit, for the batch APIs, each item in the batch is considered 1 transaction with the sole exception of BatchCancelOrders, where each cancel counts as 0.2 transactions: BatchCreateOrders BatchCancelOrders CreateOrder CancelOrder AmendOrder DecreaseOrder We reserve the right to downgrade your API rate limit tier from Prime and Premier when you have shown lack of activity in the previous period. At any time, any Member that uses FIX or is at the highest possible API tier is eligible for an upgrade to its rate limit upon demonstration that such a tier is necessary for its bona fide market activity.](#rate-limits-and-tiers-api-documentationrate-limits-and-tiers​access-tiers-tierreadwritebasic20-per-second10-per-secondadvanced30-per-second30-per-secondpremier100-per-second100-per-secondprime400-per-second400-per-second-qualification-for-tiers-basic-completing-signup-advanced-completing-httpskalshitypeformcomadvanced-api-premier-375-of-exchange-traded-volume-in-a-given-month-prime-75-of-exchange-traded-volume-in-a-given-month-in-addition-to-the-volume-targets-technical-competency-is-a-requirement-for-premierprime-access-before-providing-access-to-the-premierprime-tiers-the-exchange-will-establish-that-the-tradertrading-entity-has-the-following-requirements-met-knowledge-of-common-security-practices-for-api-usage-proficiency-in-setting-up-monitoring-for-api-usage-and-ability-to-monitor-api-usage-in-near-real-time-understanding-and-implementation-of-rate-limiting-and-throttling-mechanisms-imposed-by-the-api-and-the-ability-to-self-limit-load-awareness-of-legal-and-compliance-aspects-related-to-api-usage-only-the-following-apis-fall-under-the-write-limit-for-the-batch-apis-each-item-in-the-batch-is-considered-1-transaction-with-the-sole-exception-of-batchcancelorders-where-each-cancel-counts-as-02-transactions-batchcreateorders-batchcancelorders-createorder-cancelorder-amendorder-decreaseorder-we-reserve-the-right-to-downgrade-your-api-rate-limit-tier-from-prime-and-premier-when-you-have-shown-lack-of-activity-in-the-previous-period-at-any-time-any-member-that-uses-fix-or-is-at-the-highest-possible-api-tier-is-eligible-for-an-upgrade-to-its-rate-limit-upon-demonstration-that-such-a-tier-is-necessary-for-its-bona-fide-market-activity)
          - [Subpenny Pricing - API DocumentationSubpenny Pricing​Format Copy{ "price": 12, // legacy: cents "price_dollars": "0.1200" // new: fixed-point dollars } Starting soon in the API, you will begin to see prices and money represented in 2 different formats: integer cents (legacy) and fixed-point dollars (new). A fixed-point dollar is a string bearing a fixed-point representation of money accurate to at least 4 decimal points. ​Motivation Subpenny pricing will allow for more accurate pricing and the tail end of markets where likelihood of a given event are close to 100% or 0%. ​Status Currently the minimum tick size on all markets is still 1 cent. Additionally, all prices and money fields will continue to be available in the legacy integer cents format. However, in the near future we will be introducing sub-penny pricing on orders. As such, we will eventually the legacy integer cents format. Therefore, please update systems to parse the new fixed-point dollars fields and prepare for subpenny precision.](#subpenny-pricing-api-documentationsubpenny-pricing​format-copy-price-12-legacy-cents-price_dollars-01200-new-fixed-point-dollars-starting-soon-in-the-api-you-will-begin-to-see-prices-and-money-represented-in-2-different-formats-integer-cents-legacy-and-fixed-point-dollars-new-a-fixed-point-dollar-is-a-string-bearing-a-fixed-point-representation-of-money-accurate-to-at-least-4-decimal-points-​motivation-subpenny-pricing-will-allow-for-more-accurate-pricing-and-the-tail-end-of-markets-where-likelihood-of-a-given-event-are-close-to-100-or-0-​status-currently-the-minimum-tick-size-on-all-markets-is-still-1-cent-additionally-all-prices-and-money-fields-will-continue-to-be-available-in-the-legacy-integer-cents-format-however-in-the-near-future-we-will-be-introducing-sub-penny-pricing-on-orders-as-such-we-will-eventually-the-legacy-integer-cents-format-therefore-please-update-systems-to-parse-the-new-fixed-point-dollars-fields-and-prepare-for-subpenny-precision)

---

## Making Your First Request - API DocumentationMaking Your First RequestTo make your request we recommend testing a public endpoint like GetMarkets. As you explore our other endpoints, you’ll notice some endpoints return an authentication_error. If you want to experiment with these endpoints, you will need to get API Keys. You may also want to sign up for a demo account to test without real funds. The following resources might help you on your journey to exploring Kalshi’s markets: Python starter code bare-bones API interactions with authentication examples (also see API Keys) Python starter code 2 this Python client is not maintained, but you may still find it useful as a reference for how to call certain endpoints Discord and check out #dev and #support

To make your request we recommend testing a public endpoint like [GetMarkets](https://docs.kalshi.com/api-reference/market/get-markets). As you explore our other endpoints, you’ll notice some endpoints return an authentication\_error. If you want to experiment with these endpoints, you will need to get [API Keys](https://docs.kalshi.com/getting_started/api_keys). You may also want to sign up for a demo account to test without real funds.The following resources might help you on your journey to exploring Kalshi’s markets:

- [**Python starter code**](https://github.com/Kalshi/kalshi-starter-code-python/tree/main) bare-bones API interactions with authentication examples (also see [API Keys](https://docs.kalshi.com/getting_started/api_keys))
- [**Python starter code 2**](https://kalshi-public-docs.s3.amazonaws.com/KalshiAPIStarterCodeWithApiKey.zip) this Python client is not maintained, but you may still find it useful as a reference for how to call certain endpoints
- [**Discord**](https://discord.gg/kalshi) and check out #dev and #support


---

### Quick Start: Market Data - API DocumentationQuick Start: Market DataThis guide will walk you through accessing Kalshi’s public market data endpoints without authentication. You’ll learn how to retrieve series information, events, markets, and orderbook data for the popular “Who will have a higher net approval” market. ​Making Unauthenticated Requests Kalshi provides several public endpoints that don’t require API keys. These endpoints allow you to access market data directly from our production servers at https://api.elections.kalshi.com/trade-api/v2. Note about the API URL: Despite the “elections” subdomain, api.elections.kalshi.com provides access to ALL Kalshi markets - not just election-related ones. This includes markets on economics, climate, technology, entertainment, and more. No authentication headers are required for the endpoints in this guide. You can start making requests immediately! ​Step 1: Get Series Information Let’s start by fetching information about the KXHIGHNY series (Highest temperature in NYC today?). This series tracks the highest temperature recorded in Central Park, New York on a given day. We’ll use the Get Series endpoint. PythonJavaScriptcURLCopyimport requests # Get series information for KXHIGHNY url = "https://api.elections.kalshi.com/trade-api/v2/series/KXHIGHNY" response = requests.get(url) series_data = response.json() print(f"Series Title: {series_data['series']['title']}") print(f"Frequency: {series_data['series']['frequency']}") print(f"Category: {series_data['series']['category']}") ​Step 2: Get Today’s Events and Markets Now that we have the series information, let’s get the markets for this series. We’ll use the Get Markets endpoint with the series ticker filter to find all active markets. PythonJavaScriptCopy# Get all markets for the KXHIGHNY series markets_url = f"https://api.elections.kalshi.com/trade-api/v2/markets?series_ticker=KXHIGHNY&status=open" markets_response = requests.get(markets_url) markets_data = markets_response.json() print(f"\nActive markets in KXHIGHNY series:") for market in markets_data['markets']: print(f"- {market['ticker']}: {market['title']}") print(f" Event: {market['event_ticker']}") print(f" Yes Price: {market['yes_price']}¢ | Volume: {market['volume']}") print() # Get details for a specific event if you have its ticker if markets_data['markets']: # Let's get details for the first market's event event_ticker = markets_data['markets'][0]['event_ticker'] event_url = f"https://api.elections.kalshi.com/trade-api/v2/events/{event_ticker}" event_response = requests.get(event_url) event_data = event_response.json() print(f"Event Details:") print(f"Title: {event_data['event']['title']}") print(f"Category: {event_data['event']['category']}") You can view these markets in the Kalshi UI at: https://kalshi.com/markets/kxhighny ​Step 3: Get Orderbook Data Now let’s fetch the orderbook for a specific market to see the current bids and asks using the Get Market Orderbook endpoint. PythonJavaScriptCopy# Get orderbook for a specific market # Replace with an actual market ticker from the markets list market_ticker = markets_data['markets'][0]['ticker'] orderbook_url = f"https://api.elections.kalshi.com/trade-api/v2/markets/{market_ticker}/orderbook" orderbook_response = requests.get(orderbook_url) orderbook_data = orderbook_response.json() print(f"\nOrderbook for {market_ticker}:") print("YES BIDS:") for bid in orderbook_data['orderbook']['yes'][:5]: # Show top 5 print(f" Price: {bid[0]}¢, Quantity: {bid[1]}") print("\nNO BIDS:") for bid in orderbook_data['orderbook']['no'][:5]: # Show top 5 print(f" Price: {bid[0]}¢, Quantity: {bid[1]}") ​Working with Large Datasets The Kalshi API uses cursor-based pagination to handle large datasets efficiently. To learn more about navigating through paginated responses, see our Understanding Pagination guide. ​Understanding Orderbook Responses Kalshi’s orderbook structure is unique due to the nature of binary prediction markets. The API only returns bids (not asks) because of the reciprocal relationship between YES and NO positions. To learn more about orderbook responses and why they work this way, see our Orderbook Responses guide. ​Next Steps Now that you understand how to access market data without authentication, you can: Explore other public series and events Build real-time market monitoring tools Create market analysis dashboards Set up a WebSocket connection for live updates (requires authentication) For authenticated endpoints that allow trading and portfolio management, check out our API Keys guide.

This guide will walk you through accessing Kalshi’s public market data endpoints without authentication. You’ll learn how to retrieve series information, events, markets, and orderbook data for the popular “Who will have a higher net approval” market.

## [​](\#making-unauthenticated-requests)  Making Unauthenticated Requests

Kalshi provides several public endpoints that don’t require API keys. These endpoints allow you to access market data directly from our production servers at `https://api.elections.kalshi.com/trade-api/v2`.

**Note about the API URL**: Despite the “elections” subdomain, `api.elections.kalshi.com` provides access to ALL Kalshi markets - not just election-related ones. This includes markets on economics, climate, technology, entertainment, and more.

No authentication headers are required for the endpoints in this guide. You can start making requests immediately!

## [​](\#step-1:-get-series-information)  Step 1: Get Series Information

Let’s start by fetching information about the KXHIGHNY series ( [Highest temperature in NYC today?](https://kalshi.com/markets/kxhighny/highest-temperature-in-nyc)). This series tracks the highest temperature recorded in Central Park, New York on a given day. We’ll use the [Get Series](/api-reference/market/get-series) endpoint.

Python

JavaScript

cURL

Copy

```
import requests

# Get series information for KXHIGHNY
url = "https://api.elections.kalshi.com/trade-api/v2/series/KXHIGHNY"
response = requests.get(url)
series_data = response.json()

print(f"Series Title: {series_data['series']['title']}")
print(f"Frequency: {series_data['series']['frequency']}")
print(f"Category: {series_data['series']['category']}")

```

## [​](\#step-2:-get-today’s-events-and-markets)  Step 2: Get Today’s Events and Markets

Now that we have the series information, let’s get the markets for this series. We’ll use the [Get Markets](/api-reference/market/get-markets) endpoint with the series ticker filter to find all active markets.

Python

JavaScript

Copy

```
# Get all markets for the KXHIGHNY series
markets_url = f"https://api.elections.kalshi.com/trade-api/v2/markets?series_ticker=KXHIGHNY&status=open"
markets_response = requests.get(markets_url)
markets_data = markets_response.json()

print(f"\nActive markets in KXHIGHNY series:")
for market in markets_data['markets']:
    print(f"- {market['ticker']}: {market['title']}")
    print(f"  Event: {market['event_ticker']}")
    print(f"  Yes Price: {market['yes_price']}¢ | Volume: {market['volume']}")
    print()

# Get details for a specific event if you have its ticker
if markets_data['markets']:
    # Let's get details for the first market's event
    event_ticker = markets_data['markets'][0]['event_ticker']
    event_url = f"https://api.elections.kalshi.com/trade-api/v2/events/{event_ticker}"
    event_response = requests.get(event_url)
    event_data = event_response.json()

    print(f"Event Details:")
    print(f"Title: {event_data['event']['title']}")
    print(f"Category: {event_data['event']['category']}")

```

You can view these markets in the Kalshi UI at: [https://kalshi.com/markets/kxhighny](https://kalshi.com/markets/kxhighny)

## [​](\#step-3:-get-orderbook-data)  Step 3: Get Orderbook Data

Now let’s fetch the orderbook for a specific market to see the current bids and asks using the [Get Market Orderbook](/api-reference/market/get-market-order-book) endpoint.

Python

JavaScript

Copy

```
# Get orderbook for a specific market
# Replace with an actual market ticker from the markets list
market_ticker = markets_data['markets'][0]['ticker']
orderbook_url = f"https://api.elections.kalshi.com/trade-api/v2/markets/{market_ticker}/orderbook"

orderbook_response = requests.get(orderbook_url)
orderbook_data = orderbook_response.json()

print(f"\nOrderbook for {market_ticker}:")
print("YES BIDS:")
for bid in orderbook_data['orderbook']['yes'][:5]:  # Show top 5
    print(f"  Price: {bid[0]}¢, Quantity: {bid[1]}")

print("\nNO BIDS:")
for bid in orderbook_data['orderbook']['no'][:5]:  # Show top 5
    print(f"  Price: {bid[0]}¢, Quantity: {bid[1]}")

```

## [​](\#working-with-large-datasets)  Working with Large Datasets

The Kalshi API uses cursor-based pagination to handle large datasets efficiently. To learn more about navigating through paginated responses, see our [Understanding Pagination](/getting_started/pagination) guide.

## [​](\#understanding-orderbook-responses)  Understanding Orderbook Responses

Kalshi’s orderbook structure is unique due to the nature of binary prediction markets. The API only returns bids (not asks) because of the reciprocal relationship between YES and NO positions. To learn more about orderbook responses and why they work this way, see our [Orderbook Responses](/getting_started/orderbook_responses) guide.

## [​](\#next-steps)  Next Steps

Now that you understand how to access market data without authentication, you can:

1. Explore other public series and events
2. Build real-time market monitoring tools
3. Create market analysis dashboards
4. Set up a WebSocket connection for live updates (requires authentication)

For authenticated endpoints that allow trading and portfolio management, check out our [API Keys guide](/getting_started/api_keys).


---

#### Understanding Pagination - API DocumentationUnderstanding PaginationThe Kalshi API uses cursor-based pagination to help you efficiently navigate through large datasets. This guide explains how pagination works and provides examples for handling paginated responses. ​How Pagination Works When making requests to list endpoints (like /markets, /events, or /series), the API returns results in pages to keep response sizes manageable. Each page contains: Data array: The actual items for the current page (markets, events, etc.) Cursor field: A token that points to the next page of results Limit: The maximum number of items per page (default: 100) ​Using Cursors To paginate through results: Make your initial request without a cursor Check if the response includes a cursor field If a cursor exists, make another request with ?cursor={cursor_value} Continue until the cursor is null (no more pages) ​Example: Paginating Through Markets PythonJavaScriptCopyimport requests def get_all_markets(series_ticker): """Fetch all markets for a series, handling pagination""" all_markets = [] cursor = None base_url = "https://api.elections.kalshi.com/trade-api/v2/markets" while True: # Build URL with cursor if we have one url = f"{base_url}?series_ticker={series_ticker}&limit=100" if cursor: url += f"&cursor={cursor}" response = requests.get(url) data = response.json() # Add markets from this page all_markets.extend(data['markets']) # Check if there are more pages cursor = data.get('cursor') if not cursor: break print(f"Fetched {len(data['markets'])} markets, total: {len(all_markets)}") return all_markets # Example usage markets = get_all_markets("KXHIGHNY") print(f"Total markets found: {len(markets)}") ​Pagination Parameters Most list endpoints support these pagination parameters: cursor: Token from previous response to get the next page limit: Number of items per page (typically 1-100, default: 100) ​Best Practices Handle rate limits: When paginating through large datasets, be mindful of rate limits Set appropriate limits: Use smaller page sizes if you only need a few items Cache results: Store paginated data locally to avoid repeated API calls Check for changes: Data can change between requests, so consider implementing refresh logic ​Endpoints Supporting Pagination The following endpoints support cursor-based pagination: Get Markets - /markets Get Events - /events Get Series - /series Get Trades - /markets/trades Get Portfolio History - /portfolio/history Get Fills - /portfolio/fills Get Orders - /portfolio/orders ​Common Patterns ​Fetching Recent Items If you only need recent items, you can limit results without pagination: Copy# Get just the 10 most recent markets url = "https://api.elections.kalshi.com/trade-api/v2/markets?limit=10&status=open" ​Filtering While Paginating You can combine filters with pagination: Copy# Get all open markets for a series url = f"{base_url}?series_ticker={ticker}&status=open&limit=100&cursor={cursor}" ​Detecting New Items To check for new items since your last fetch: Store the first item’s ID or timestamp from your previous fetch Paginate through results until you find that item Everything before it is new ​Next Steps Now that you understand pagination, you can efficiently work with large datasets in the Kalshi API. For more details on specific endpoints, check the API Reference.

The Kalshi API uses cursor-based pagination to help you efficiently navigate through large datasets. This guide explains how pagination works and provides examples for handling paginated responses.

## [​](\#how-pagination-works)  How Pagination Works

When making requests to list endpoints (like `/markets`, `/events`, or `/series`), the API returns results in pages to keep response sizes manageable. Each page contains:

- **Data array**: The actual items for the current page (markets, events, etc.)
- **Cursor field**: A token that points to the next page of results
- **Limit**: The maximum number of items per page (default: 100)

## [​](\#using-cursors)  Using Cursors

To paginate through results:

1. Make your initial request without a cursor
2. Check if the response includes a `cursor` field
3. If a cursor exists, make another request with `?cursor={cursor_value}`
4. Continue until the cursor is `null` (no more pages)

## [​](\#example:-paginating-through-markets)  Example: Paginating Through Markets

Python

JavaScript

Copy

```
import requests

def get_all_markets(series_ticker):
    """Fetch all markets for a series, handling pagination"""
    all_markets = []
    cursor = None
    base_url = "https://api.elections.kalshi.com/trade-api/v2/markets"

    while True:
        # Build URL with cursor if we have one
        url = f"{base_url}?series_ticker={series_ticker}&limit=100"
        if cursor:
            url += f"&cursor={cursor}"

        response = requests.get(url)
        data = response.json()

        # Add markets from this page
        all_markets.extend(data['markets'])

        # Check if there are more pages
        cursor = data.get('cursor')
        if not cursor:
            break

        print(f"Fetched {len(data['markets'])} markets, total: {len(all_markets)}")

    return all_markets

# Example usage
markets = get_all_markets("KXHIGHNY")
print(f"Total markets found: {len(markets)}")

```

## [​](\#pagination-parameters)  Pagination Parameters

Most list endpoints support these pagination parameters:

- **`cursor`**: Token from previous response to get the next page
- **`limit`**: Number of items per page (typically 1-100, default: 100)

## [​](\#best-practices)  Best Practices

1. **Handle rate limits**: When paginating through large datasets, be mindful of [rate limits](/getting_started/rate_limits)
2. **Set appropriate limits**: Use smaller page sizes if you only need a few items
3. **Cache results**: Store paginated data locally to avoid repeated API calls
4. **Check for changes**: Data can change between requests, so consider implementing refresh logic

## [​](\#endpoints-supporting-pagination)  Endpoints Supporting Pagination

The following endpoints support cursor-based pagination:

- [Get Markets](/api-reference/market/get-markets) \- `/markets`
- [Get Events](/api-reference/market/get-events) \- `/events`
- [Get Series](/api-reference/market/get-series) \- `/series`
- [Get Trades](/api-reference/market/get-trades) \- `/markets/trades`
- [Get Portfolio History](/api-reference/portfolio/get-portfolio-history) \- `/portfolio/history`
- [Get Fills](/api-reference/portfolio/get-fills) \- `/portfolio/fills`
- [Get Orders](/api-reference/portfolio/get-orders) \- `/portfolio/orders`

## [​](\#common-patterns)  Common Patterns

### [​](\#fetching-recent-items)  Fetching Recent Items

If you only need recent items, you can limit results without pagination:

Copy

```
# Get just the 10 most recent markets
url = "https://api.elections.kalshi.com/trade-api/v2/markets?limit=10&status=open"

```

### [​](\#filtering-while-paginating)  Filtering While Paginating

You can combine filters with pagination:

Copy

```
# Get all open markets for a series
url = f"{base_url}?series_ticker={ticker}&status=open&limit=100&cursor={cursor}"

```

### [​](\#detecting-new-items)  Detecting New Items

To check for new items since your last fetch:

1. Store the first item’s ID or timestamp from your previous fetch
2. Paginate through results until you find that item
3. Everything before it is new

## [​](\#next-steps)  Next Steps

Now that you understand pagination, you can efficiently work with large datasets in the Kalshi API. For more details on specific endpoints, check the [API Reference](/api-reference).


---

#### Quick Start: Authenticated Requests - API DocumentationQuick Start: Authenticated RequestsThis guide shows you how to make authenticated requests to the Kalshi API in three simple steps. ​Step 1: Get Your API Keys Log in to your Kalshi account (demo or production) Navigate to Account & security → API Keys Click Create Key Save both: Private Key: Downloaded as a .key file API Key ID: Displayed on screen (looks like a952bcbe-ec3b-4b5b-b8f9-11dae589608c) Your private key cannot be retrieved after this page is closed. Store it securely! ​Step 2: Set Up Your Request Every authenticated request to Kalshi requires three headers: HeaderDescriptionExampleKALSHI-ACCESS-KEYYour API Key IDa952bcbe-ec3b-4b5b-b8f9-11dae589608cKALSHI-ACCESS-TIMESTAMPCurrent time in milliseconds1703123456789KALSHI-ACCESS-SIGNATURERequest signature (see below)base64_encoded_signature ​How to Create the Signature The signature proves you own the private key. Here’s how it works: Create a message string: Concatenate timestamp + HTTP_METHOD + path Example: 1703123456789GET/trade-api/v2/portfolio/balance Important: Use the path without query parameters. For /portfolio/orders?limit=5, sign only /trade-api/v2/portfolio/orders Sign with your private key: Use RSA-PSS with SHA256 Encode as base64: Convert the signature to base64 string Here’s the signing process in Python: Copyimport base64 from cryptography.hazmat.primitives import hashes from cryptography.hazmat.primitives.asymmetric import padding def sign_request(private_key, timestamp, method, path): # Strip query parameters from path before signing path_without_query = path.split('?')[0] # Create the message to sign message = f"{timestamp}{method}{path_without_query}".encode('utf-8') # Sign with RSA-PSS signature = private_key.sign( message, padding.PSS( mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH ), hashes.SHA256() ) # Return base64 encoded return base64.b64encode(signature).decode('utf-8') ​Step 3: Get Your Balance Now let’s make your first authenticated request to get your account balance: Copyimport requests import datetime # Set up the request timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) method = "GET" path = "/trade-api/v2/portfolio/balance" # Create signature (using function from Step 2) signature = sign_request(private_key, timestamp, method, path) # Make the request headers = { 'KALSHI-ACCESS-KEY': 'your-api-key-id', 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp } response = requests.get('https://demo-api.kalshi.co' + path, headers=headers) balance = response.json() print(f"Your balance: ${balance['balance'] / 100:.2f}") ​Complete Working Example Here’s the minimal code to get your balance: Copyimport requests import datetime import base64 from cryptography.hazmat.primitives import serialization, hashes from cryptography.hazmat.backends import default_backend from cryptography.hazmat.primitives.asymmetric import padding # Configuration API_KEY_ID = 'your-api-key-id-here' PRIVATE_KEY_PATH = 'path/to/your/kalshi-key.key' BASE_URL = 'https://demo-api.kalshi.co' # or 'https://api.kalshi.com' for production def load_private_key(key_path): """Load the private key from file.""" with open(key_path, "rb") as f: return serialization.load_pem_private_key(f.read(), password=None, backend=default_backend()) def create_signature(private_key, timestamp, method, path): """Create the request signature.""" # Strip query parameters before signing path_without_query = path.split('?')[0] message = f"{timestamp}{method}{path_without_query}".encode('utf-8') signature = private_key.sign( message, padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH), hashes.SHA256() ) return base64.b64encode(signature).decode('utf-8') def get(private_key, api_key_id, path, base_url=BASE_URL): """Make an authenticated GET request to the Kalshi API.""" timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) signature = create_signature(private_key, timestamp, "GET", path) headers = { 'KALSHI-ACCESS-KEY': api_key_id, 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp } return requests.get(base_url + path, headers=headers) # Load private key private_key = load_private_key(PRIVATE_KEY_PATH) # Get balance response = get(private_key, API_KEY_ID, "/trade-api/v2/portfolio/balance") print(f"Your balance: ${response.json()['balance'] / 100:.2f}") ​Common Issues ProblemSolution401 UnauthorizedCheck your API Key ID and private key file pathSignature errorEnsure timestamp is in milliseconds (not seconds)Path not foundPath must start with /trade-api/v2/Signature error with query paramsStrip query parameters before signing (use path.split('?')[0]) ​Next Steps Now you can make authenticated requests! Try these endpoints: /trade-api/v2/portfolio/positions - Get your positions /trade-api/v2/portfolio/orders - View your orders /trade-api/v2/markets - Browse available markets For more details, see the Complete Order Lifecycle guide or explore the API Reference.

This guide shows you how to make authenticated requests to the Kalshi API in three simple steps.

## [​](\#step-1:-get-your-api-keys)  Step 1: Get Your API Keys

1. Log in to your Kalshi account ( [demo](https://demo.kalshi.com) or [production](https://kalshi.com))
2. Navigate to **Account & security** → **API Keys**
3. Click **Create Key**
4. Save both:
   - **Private Key**: Downloaded as a `.key` file
   - **API Key ID**: Displayed on screen (looks like `a952bcbe-ec3b-4b5b-b8f9-11dae589608c`)

Your private key cannot be retrieved after this page is closed. Store it securely!

## [​](\#step-2:-set-up-your-request)  Step 2: Set Up Your Request

Every authenticated request to Kalshi requires three headers:

| Header | Description | Example |
| --- | --- | --- |
| `KALSHI-ACCESS-KEY` | Your API Key ID | `a952bcbe-ec3b-4b5b-b8f9-11dae589608c` |
| `KALSHI-ACCESS-TIMESTAMP` | Current time in milliseconds | `1703123456789` |
| `KALSHI-ACCESS-SIGNATURE` | Request signature (see below) | `base64_encoded_signature` |

### [​](\#how-to-create-the-signature)  How to Create the Signature

The signature proves you own the private key. Here’s how it works:

1. **Create a message string**: Concatenate `timestamp + HTTP_METHOD + path`   - Example: `1703123456789GET/trade-api/v2/portfolio/balance`
   - **Important**: Use the path **without query parameters**. For `/portfolio/orders?limit=5`, sign only `/trade-api/v2/portfolio/orders`
2. **Sign with your private key**: Use RSA-PSS with SHA256
3. **Encode as base64**: Convert the signature to base64 string

Here’s the signing process in Python:

Copy

```
import base64
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding

def sign_request(private_key, timestamp, method, path):
    # Strip query parameters from path before signing
    path_without_query = path.split('?')[0]

    # Create the message to sign
    message = f"{timestamp}{method}{path_without_query}".encode('utf-8')

    # Sign with RSA-PSS
    signature = private_key.sign(
        message,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.DIGEST_LENGTH
        ),
        hashes.SHA256()
    )

    # Return base64 encoded
    return base64.b64encode(signature).decode('utf-8')

```

## [​](\#step-3:-get-your-balance)  Step 3: Get Your Balance

Now let’s make your first authenticated request to get your account balance:

Copy

```
import requests
import datetime

# Set up the request
timestamp = str(int(datetime.datetime.now().timestamp() * 1000))
method = "GET"
path = "/trade-api/v2/portfolio/balance"

# Create signature (using function from Step 2)
signature = sign_request(private_key, timestamp, method, path)

# Make the request
headers = {
    'KALSHI-ACCESS-KEY': 'your-api-key-id',
    'KALSHI-ACCESS-SIGNATURE': signature,
    'KALSHI-ACCESS-TIMESTAMP': timestamp
}

response = requests.get('https://demo-api.kalshi.co' + path, headers=headers)
balance = response.json()

print(f"Your balance: ${balance['balance'] / 100:.2f}")

```

## [​](\#complete-working-example)  Complete Working Example

Here’s the minimal code to get your balance:

Copy

```
import requests
import datetime
import base64
from cryptography.hazmat.primitives import serialization, hashes
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives.asymmetric import padding

# Configuration
API_KEY_ID = 'your-api-key-id-here'
PRIVATE_KEY_PATH = 'path/to/your/kalshi-key.key'
BASE_URL = 'https://demo-api.kalshi.co'  # or 'https://api.kalshi.com' for production

def load_private_key(key_path):
    """Load the private key from file."""
    with open(key_path, "rb") as f:
        return serialization.load_pem_private_key(f.read(), password=None, backend=default_backend())

def create_signature(private_key, timestamp, method, path):
    """Create the request signature."""
    # Strip query parameters before signing
    path_without_query = path.split('?')[0]
    message = f"{timestamp}{method}{path_without_query}".encode('utf-8')
    signature = private_key.sign(
        message,
        padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH),
        hashes.SHA256()
    )
    return base64.b64encode(signature).decode('utf-8')

def get(private_key, api_key_id, path, base_url=BASE_URL):
    """Make an authenticated GET request to the Kalshi API."""
    timestamp = str(int(datetime.datetime.now().timestamp() * 1000))
    signature = create_signature(private_key, timestamp, "GET", path)

    headers = {
        'KALSHI-ACCESS-KEY': api_key_id,
        'KALSHI-ACCESS-SIGNATURE': signature,
        'KALSHI-ACCESS-TIMESTAMP': timestamp
    }

    return requests.get(base_url + path, headers=headers)

# Load private key
private_key = load_private_key(PRIVATE_KEY_PATH)

# Get balance
response = get(private_key, API_KEY_ID, "/trade-api/v2/portfolio/balance")
print(f"Your balance: ${response.json()['balance'] / 100:.2f}")

```

## [​](\#common-issues)  Common Issues

| Problem | Solution |
| --- | --- |
| 401 Unauthorized | Check your API Key ID and private key file path |
| Signature error | Ensure timestamp is in milliseconds (not seconds) |
| Path not found | Path must start with `/trade-api/v2/` |
| Signature error with query params | Strip query parameters before signing (use `path.split('?')[0]`) |

## [​](\#next-steps)  Next Steps

Now you can make authenticated requests! Try these endpoints:

- `/trade-api/v2/portfolio/positions` \- Get your positions
- `/trade-api/v2/portfolio/orders` \- View your orders
- `/trade-api/v2/markets` \- Browse available markets

For more details, see the [Complete Order Lifecycle](./complete_order_lifecycle) guide or explore the [API Reference](/api-reference).


---

##### Page Not Found404Page Not FoundWe couldn't find the page. Maybe you were looking for one of these pages below?Quick Start: Create your first orderGet OrdersQuick Start: Authenticated RequestsPage Not Found

404

# Page Not Found

We couldn't find the page. Maybe you were looking for one of these pages below?

[Quick Start: Create your first order](/getting_started/quick_start_create_order#quick-start-create-your-first-order) [Get Orders](/api-reference/orders/get-orders#) [Quick Start: Authenticated Requests](/getting_started/quick_start_authenticated_requests#next-steps)


---

##### Test In The Demo Environment - API DocumentationTest In The Demo EnvironmentFor testing purposes, Kalshi offers a demo environment with mock funds. You can access the Demo environment at https://demo.kalshi.co/. For safety, credentials are not shared between this environment and production. To set up a Kalshi Demo account, follow this step-by-step tutorial. Demo’s API root is https://demo-api.kalshi.co/trade-api/v2.

For testing purposes, Kalshi offers a _demo_ environment with mock funds. You can access the Demo environment at [https://demo.kalshi.co/](https://demo.kalshi.co/). For safety, credentials are not shared between this environment and production.To set up a Kalshi Demo account, [follow this step-by-step tutorial](https://docs.google.com/presentation/d/e/2PACX-1vRvhUAqRBYzJmt7JCinMXmu6KVWkj-cc7ikDXGConmqjcv4mnlJacgHPcZJ20fWWnrYdubn-oczclKP/pub?start=false&loop=false&delayms=3000&slide=id.p).Demo’s API root is `https://demo-api.kalshi.co/trade-api/v2`.


---

##### Quick Start: Create your first order - API DocumentationQuick Start: Create your first orderThis guide will walk you through the complete lifecycle of placing and managing orders on Kalshi. ​Prerequisites Before you begin, you’ll need: A Kalshi account with API access configured Python with the requests and cryptography libraries installed Your authentication functions set up (see our authentication guide) This guide assumes you have the authentication code from our authentication guide, including the get() function for making authenticated requests. ​Step 1: Find an Open Market First, let’s find an open market to trade on. Copy# Get the first open market (no auth required for public market data) response = requests.get('https://demo-api.kalshi.co/trade-api/v2/markets?limit=1&status=open') market = response.json()['markets'][0] print(f"Selected market: {market['ticker']}") print(f"Title: {market['title']}") ​Step 2: Place a Buy Order Now let’s place an order to buy 1 YES contract for 1 cent (limit order). We’ll use a client_order_id to deduplicate orders - this allows you to identify duplicate orders before receiving the server-generated order_id in the response. Copyimport uuid def post(private_key, api_key_id, path, data, base_url=BASE_URL): """Make an authenticated POST request to the Kalshi API.""" timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) signature = create_signature(private_key, timestamp, "POST", path) headers = { 'KALSHI-ACCESS-KEY': api_key_id, 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp, 'Content-Type': 'application/json' } return requests.post(base_url + path, headers=headers, json=data) # Place a buy order for 1 YES contract at 1 cent order_data = { "ticker": market['ticker'], "action": "buy", "side": "yes", "count": 1, "type": "limit", "yes_price": 1, "client_order_id": str(uuid.uuid4()) # Unique ID for deduplication } response = post(private_key, API_KEY_ID, '/trade-api/v2/portfolio/orders', order_data) if response.status_code == 201: order = response.json()['order'] print(f"Order placed successfully!") print(f"Order ID: {order['order_id']}") print(f"Client Order ID: {order_data['client_order_id']}") print(f"Status: {order['status']}") else: print(f"Error: {response.status_code} - {response.text}") ​Complete Example Script Here’s a complete script that creates your first order: Copyimport requests import uuid # Assumes you have the authentication code from the prerequisites # Add POST function to your existing auth code def post(private_key, api_key_id, path, data, base_url=BASE_URL): """Make an authenticated POST request to the Kalshi API.""" timestamp = str(int(datetime.datetime.now().timestamp() * 1000)) signature = create_signature(private_key, timestamp, "POST", path) headers = { 'KALSHI-ACCESS-KEY': api_key_id, 'KALSHI-ACCESS-SIGNATURE': signature, 'KALSHI-ACCESS-TIMESTAMP': timestamp, 'Content-Type': 'application/json' } return requests.post(base_url + path, headers=headers, json=data) # Step 1: Find an open market print("Finding an open market...") response = requests.get('https://demo-api.kalshi.co/trade-api/v2/markets?limit=1&status=open') market = response.json()['markets'][0] print(f"Selected: {market['ticker']} - {market['title']}") # Step 2: Place a buy order print("\nPlacing order...") client_order_id = str(uuid.uuid4()) order_data = { "ticker": market['ticker'], "action": "buy", "side": "yes", "count": 1, "type": "limit", "yes_price": 1, "client_order_id": client_order_id } response = post(private_key, API_KEY_ID, '/trade-api/v2/portfolio/orders', order_data) if response.status_code == 201: order = response.json()['order'] print(f"Order placed successfully!") print(f"Order ID: {order['order_id']}") print(f"Client Order ID: {client_order_id}") print(f"Status: {order['status']}") else: print(f"Error: {response.status_code} - {response.text}") ​Important Notes ​Client Order ID The client_order_id field is crucial for order deduplication: Generate a unique ID (like UUID4) for each order before submission If network issues occur, you can resubmit with the same client_order_id The API will reject duplicate submissions, preventing accidental double orders Store this ID locally to track orders before receiving the server’s order_id ​Error Handling Common errors and how to handle them: 401 Unauthorized: Check your API keys and signature generation 400 Bad Request: Verify your order parameters (price must be 1-99 cents) 409 Conflict: Order with this client_order_id already exists 429 Too Many Requests: You’ve hit the rate limit - slow down your requests ​Next Steps Now that you’ve created your first order, you can: Check order status using the /portfolio/orders/{order_id} endpoint List all your orders with /portfolio/orders Amend your order price or quantity using PUT /portfolio/orders/{order_id} Cancel orders using DELETE /portfolio/orders/{order_id} Implement WebSocket connections for real-time updates Build automated trading strategies For more information, check out: API Reference Documentation Python Starter Code Kalshi Discord Community

This guide will walk you through the complete lifecycle of placing and managing orders on Kalshi.

## [​](\#prerequisites)  Prerequisites

Before you begin, you’ll need:

- A Kalshi account with API access configured
- Python with the `requests` and `cryptography` libraries installed
- Your authentication functions set up (see our [authentication guide](quick_start_authenticated_requests))

This guide assumes you have the authentication code from our authentication guide, including the `get()` function for making authenticated requests.

## [​](\#step-1:-find-an-open-market)  Step 1: Find an Open Market

First, let’s find an open market to trade on.

Copy

```
# Get the first open market (no auth required for public market data)
response = requests.get('https://demo-api.kalshi.co/trade-api/v2/markets?limit=1&status=open')
market = response.json()['markets'][0]

print(f"Selected market: {market['ticker']}")
print(f"Title: {market['title']}")

```

## [​](\#step-2:-place-a-buy-order)  Step 2: Place a Buy Order

Now let’s place an order to buy 1 YES contract for 1 cent (limit order). We’ll use a `client_order_id` to deduplicate orders - this allows you to identify duplicate orders before receiving the server-generated `order_id` in the response.

Copy

```
import uuid

def post(private_key, api_key_id, path, data, base_url=BASE_URL):
    """Make an authenticated POST request to the Kalshi API."""
    timestamp = str(int(datetime.datetime.now().timestamp() * 1000))
    signature = create_signature(private_key, timestamp, "POST", path)

    headers = {
        'KALSHI-ACCESS-KEY': api_key_id,
        'KALSHI-ACCESS-SIGNATURE': signature,
        'KALSHI-ACCESS-TIMESTAMP': timestamp,
        'Content-Type': 'application/json'
    }

    return requests.post(base_url + path, headers=headers, json=data)

# Place a buy order for 1 YES contract at 1 cent
order_data = {
    "ticker": market['ticker'],
    "action": "buy",
    "side": "yes",
    "count": 1,
    "type": "limit",
    "yes_price": 1,
    "client_order_id": str(uuid.uuid4())  # Unique ID for deduplication
}

response = post(private_key, API_KEY_ID, '/trade-api/v2/portfolio/orders', order_data)

if response.status_code == 201:
    order = response.json()['order']
    print(f"Order placed successfully!")
    print(f"Order ID: {order['order_id']}")
    print(f"Client Order ID: {order_data['client_order_id']}")
    print(f"Status: {order['status']}")
else:
    print(f"Error: {response.status_code} - {response.text}")

```

## [​](\#complete-example-script)  Complete Example Script

Here’s a complete script that creates your first order:

Copy

```
import requests
import uuid
# Assumes you have the authentication code from the prerequisites

# Add POST function to your existing auth code
def post(private_key, api_key_id, path, data, base_url=BASE_URL):
    """Make an authenticated POST request to the Kalshi API."""
    timestamp = str(int(datetime.datetime.now().timestamp() * 1000))
    signature = create_signature(private_key, timestamp, "POST", path)

    headers = {
        'KALSHI-ACCESS-KEY': api_key_id,
        'KALSHI-ACCESS-SIGNATURE': signature,
        'KALSHI-ACCESS-TIMESTAMP': timestamp,
        'Content-Type': 'application/json'
    }

    return requests.post(base_url + path, headers=headers, json=data)

# Step 1: Find an open market
print("Finding an open market...")
response = requests.get('https://demo-api.kalshi.co/trade-api/v2/markets?limit=1&status=open')
market = response.json()['markets'][0]
print(f"Selected: {market['ticker']} - {market['title']}")

# Step 2: Place a buy order
print("\nPlacing order...")
client_order_id = str(uuid.uuid4())
order_data = {
    "ticker": market['ticker'],
    "action": "buy",
    "side": "yes",
    "count": 1,
    "type": "limit",
    "yes_price": 1,
    "client_order_id": client_order_id
}

response = post(private_key, API_KEY_ID, '/trade-api/v2/portfolio/orders', order_data)

if response.status_code == 201:
    order = response.json()['order']
    print(f"Order placed successfully!")
    print(f"Order ID: {order['order_id']}")
    print(f"Client Order ID: {client_order_id}")
    print(f"Status: {order['status']}")
else:
    print(f"Error: {response.status_code} - {response.text}")

```

## [​](\#important-notes)  Important Notes

### [​](\#client-order-id)  Client Order ID

The `client_order_id` field is crucial for order deduplication:

- Generate a unique ID (like UUID4) for each order before submission
- If network issues occur, you can resubmit with the same `client_order_id`
- The API will reject duplicate submissions, preventing accidental double orders
- Store this ID locally to track orders before receiving the server’s `order_id`

### [​](\#error-handling)  Error Handling

Common errors and how to handle them:

- `401 Unauthorized`: Check your API keys and signature generation
- `400 Bad Request`: Verify your order parameters (price must be 1-99 cents)
- `409 Conflict`: Order with this `client_order_id` already exists
- `429 Too Many Requests`: You’ve hit the rate limit - slow down your requests

## [​](\#next-steps)  Next Steps

Now that you’ve created your first order, you can:

- Check order status using the `/portfolio/orders/{order_id}` endpoint
- List all your orders with `/portfolio/orders`
- Amend your order price or quantity using PUT `/portfolio/orders/{order_id}`
- Cancel orders using DELETE `/portfolio/orders/{order_id}`
- Implement WebSocket connections for real-time updates
- Build automated trading strategies

For more information, check out:

- [API Reference Documentation](https://docs.kalshi.com/api-reference)
- [Python Starter Code](https://github.com/Kalshi/kalshi-starter-code-python)
- [Kalshi Discord Community](https://discord.gg/kalshi)


---

###### API Keys - API DocumentationAPI KeysThis process is the same for the demo or production environment. ​Generating an API Key ​Access the Account Settings Page: Log in to your account and navigate to the “Account Settings” page. You can typically find this option by clicking on your profile picture or account icon in the top-right corner of the application. ​Generate a New API Key In the “Profile Settings” page https://kalshi.com/account/profile, locate the “API Keys” section. Click on the “Create New API Key” button. This action will generate a new API key in the RSA_PRIVATE_KEY format. ​Store Your API Key and Key ID: After generating the key, you will be presented with: • Private Key: This is your secret key in RSA_PRIVATE_KEY format. • Key ID: This is a unique identifier associated with your private key. Important: For security reasons, the private key will not be stored by our service, and you will not be able to retrieve it again once this page is closed. Please make sure to securely copy and save the private key immediately. The key will also be downloaded as txt file with the name provided. ​Using a API Key Each request to Kalshi trading api will need to be signed with the private key generated above. The following header values will need to be provided with each request: KALSHI-ACCESS-KEY- the Key ID KALSHI-ACCESS-TIMESTAMP - the request timestamp in ms KALSHI-ACCESS-SIGNATURE- request hash signed with private key The above signature is generated by signing a concatenation of the timestamp, the HTTP method and the path. Important: When signing requests, use the path without query parameters. For example, if your request is to /trade-api/v2/portfolio/orders?limit=5, sign only /trade-api/v2/portfolio/orders (strip the ? and everything after it). Sample code for generating the required headers is below (alternatively use our example code here): ​Python Load the private key stored in a file Copyfrom cryptography.hazmat.primitives import serialization from cryptography.hazmat.backends import default_backend def load_private_key_from_file(file_path): with open(file_path, "rb") as key_file: private_key = serialization.load_pem_private_key( key_file.read(), password=None, # or provide a password if your key is encrypted backend=default_backend() ) return private_key Sign text with private key Copyimport base64 from cryptography.hazmat.primitives import hashes from cryptography.hazmat.primitives.asymmetric import padding, rsa from cryptography.exceptions import InvalidSignature def sign_pss_text(private_key: rsa.RSAPrivateKey, text: str) -> str: message = text.encode('utf-8') try: signature = private_key.sign( message, padding.PSS( mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH ), hashes.SHA256() ) return base64.b64encode(signature).decode('utf-8') except InvalidSignature as e: raise ValueError("RSA sign PSS failed") from e Send a request to Kalshi API with signed header Copyimport requests import datetime current_time = datetime.datetime.now() timestamp = current_time.timestamp() current_time_milliseconds = int(timestamp * 1000) timestampt_str = str(current_time_milliseconds) private_key = load_private_key_from_file('kalshi-key-2.key') method = "GET" base_url = 'https://demo-api.kalshi.co' path='/trade-api/v2/portfolio/balance' # Strip query parameters from path before signing path_without_query = path.split('?')[0] msg_string = timestampt_str + method + path_without_query sig = sign_pss_text(private_key, msg_string) headers = { 'KALSHI-ACCESS-KEY': 'a952bcbe-ec3b-4b5b-b8f9-11dae589608c', 'KALSHI-ACCESS-SIGNATURE': sig, 'KALSHI-ACCESS-TIMESTAMP': timestampt_str } response = requests.get(base_url + path, headers=headers) print(response.text) ​Javascript Load the private key stored in a file Copyconst fs = require('fs'); const path = require('path'); function loadPrivateKeyFromFile(filePath) { const absolutePath = path.resolve(filePath); const privateKeyPem = fs.readFileSync(absolutePath, 'utf8'); return privateKeyPem; } Sign text with private key Copyconst crypto = require('crypto'); function signPssText(privateKeyPem, text) { const sign = crypto.createSign('RSA-SHA256'); sign.update(text); sign.end(); const signature = sign.sign({ key: privateKeyPem, padding: crypto.constants.RSA_PKCS1_PSS_PADDING, saltLength: crypto.constants.RSA_PSS_SALTLEN_DIGEST, }); return signature.toString('base64'); } Send a request to Kalshi API with signed header Copyconst axios = require('axios'); const currentTimeMilliseconds = Date.now(); const timestampStr = currentTimeMilliseconds.toString(); const privateKeyPem = loadPrivateKeyFromFile('path/to/your/private-key.pem'); const method = "GET"; const baseUrl = 'https://demo-api.kalshi.co'; const path = '/trade-api/v2/portfolio/balance'; // Strip query parameters from path before signing const pathWithoutQuery = path.split('?')[0]; const msgString = timestampStr + method + pathWithoutQuery; const sig = signPssText(privateKeyPem, msgString); const headers = { 'KALSHI-ACCESS-KEY': 'your-api-key-id', 'KALSHI-ACCESS-SIGNATURE': sig, 'KALSHI-ACCESS-TIMESTAMP': timestampStr }; axios.get(baseUrl + path, { headers }) .then(response => { console.log(response.data); }) .catch(error => { console.error('Error:', error); });

This process is the same for the demo or production environment.

## [​](\#generating-an-api-key)  Generating an API Key

### [​](\#access-the-account-settings-page:)  Access the Account Settings Page:

Log in to your account and navigate to the “Account Settings” page. You can typically find this option by clicking on your profile picture or account icon in the top-right corner of the application.

### [​](\#generate-a-new-api-key)  Generate a New API Key

In the “Profile Settings” page [https://kalshi.com/account/profile](https://kalshi.com/account/profile), locate the “API Keys” section. Click on the “Create New API Key” button. This action will generate a new API key in the RSA\_PRIVATE\_KEY format.

### [​](\#store-your-api-key-and-key-id:)  Store Your API Key and Key ID:

After generating the key, you will be presented with:
• Private Key: This is your secret key in RSA\_PRIVATE\_KEY format.
• Key ID: This is a unique identifier associated with your private key.**Important**: For security reasons, the private key will not be stored by our service, and you will not be able to retrieve it again once this page is closed. Please make sure to securely copy and save the private key immediately. The key will also be downloaded as txt file with the name provided.

## [​](\#using-a-api-key)  Using a API Key

Each request to Kalshi trading api will need to be signed with the private key generated above.The following header values will need to be provided with each request:`KALSHI-ACCESS-KEY`\- the Key ID`KALSHI-ACCESS-TIMESTAMP` \- the request timestamp in ms`KALSHI-ACCESS-SIGNATURE`\- request hash signed with private keyThe above signature is generated by signing a concatenation of the timestamp, the HTTP method and the path.

**Important**: When signing requests, use the path **without query parameters**. For example, if your request is to `/trade-api/v2/portfolio/orders?limit=5`, sign only `/trade-api/v2/portfolio/orders` (strip the `?` and everything after it).

Sample code for generating the required headers is below (alternatively use our example code [here](https://github.com/Kalshi/kalshi-starter-code-python/tree/main)):

### [​](\#python)  Python

Load the private key stored in a file

Copy

```
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.backends import default_backend

def load_private_key_from_file(file_path):
    with open(file_path, "rb") as key_file:
        private_key = serialization.load_pem_private_key(
            key_file.read(),
            password=None,  # or provide a password if your key is encrypted
            backend=default_backend()
        )
    return private_key

```

Sign text with private key

Copy

```
import base64
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding, rsa
from cryptography.exceptions import InvalidSignature

def sign_pss_text(private_key: rsa.RSAPrivateKey, text: str) -> str:
    message = text.encode('utf-8')
    try:
        signature = private_key.sign(
            message,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.DIGEST_LENGTH
            ),
            hashes.SHA256()
        )
        return base64.b64encode(signature).decode('utf-8')
    except InvalidSignature as e:
        raise ValueError("RSA sign PSS failed") from e

```

Send a request to Kalshi API with signed header

Copy

```
import requests
import datetime

current_time = datetime.datetime.now()
timestamp = current_time.timestamp()
current_time_milliseconds = int(timestamp * 1000)
timestampt_str = str(current_time_milliseconds)

private_key = load_private_key_from_file('kalshi-key-2.key')

method = "GET"
base_url = 'https://demo-api.kalshi.co'
path='/trade-api/v2/portfolio/balance'

# Strip query parameters from path before signing
path_without_query = path.split('?')[0]
msg_string = timestampt_str + method + path_without_query
sig = sign_pss_text(private_key, msg_string)

headers = {
    'KALSHI-ACCESS-KEY': 'a952bcbe-ec3b-4b5b-b8f9-11dae589608c',
    'KALSHI-ACCESS-SIGNATURE': sig,
    'KALSHI-ACCESS-TIMESTAMP': timestampt_str
}

response = requests.get(base_url + path, headers=headers)

print(response.text)

```

### [​](\#javascript)  Javascript

Load the private key stored in a file

Copy

```
const fs = require('fs');
const path = require('path');

function loadPrivateKeyFromFile(filePath) {
    const absolutePath = path.resolve(filePath);
    const privateKeyPem = fs.readFileSync(absolutePath, 'utf8');
    return privateKeyPem;
}

```

Sign text with private key

Copy

```
const crypto = require('crypto');

function signPssText(privateKeyPem, text) {
    const sign = crypto.createSign('RSA-SHA256');
    sign.update(text);
    sign.end();

    const signature = sign.sign({
        key: privateKeyPem,
        padding: crypto.constants.RSA_PKCS1_PSS_PADDING,
        saltLength: crypto.constants.RSA_PSS_SALTLEN_DIGEST,
    });

    return signature.toString('base64');
}

```

Send a request to Kalshi API with signed header

Copy

```
const axios = require('axios');

const currentTimeMilliseconds = Date.now();
const timestampStr = currentTimeMilliseconds.toString();

const privateKeyPem = loadPrivateKeyFromFile('path/to/your/private-key.pem');

const method = "GET";
const baseUrl = 'https://demo-api.kalshi.co';
const path = '/trade-api/v2/portfolio/balance';

// Strip query parameters from path before signing
const pathWithoutQuery = path.split('?')[0];
const msgString = timestampStr + method + pathWithoutQuery;
const sig = signPssText(privateKeyPem, msgString);

const headers = {
    'KALSHI-ACCESS-KEY': 'your-api-key-id',
    'KALSHI-ACCESS-SIGNATURE': sig,
    'KALSHI-ACCESS-TIMESTAMP': timestampStr
};

axios.get(baseUrl + path, { headers })
    .then(response => {
        console.log(response.data);
    })
    .catch(error => {
        console.error('Error:', error);
    });

```


---

###### Orderbook Responses - API DocumentationOrderbook Responses​Getting Orderbook Data The Get Market Orderbook endpoint returns the current state of bids for a specific market. ​Request Format CopyGET /markets/{ticker}/orderbook No authentication is required for this endpoint. ​Example Request PythonJavaScriptcURLCopyimport requests # Get orderbook for a specific market market_ticker = "KXHIGHNY-24JAN01-T60" url = f"https://api.elections.kalshi.com/trade-api/v2/markets/{market_ticker}/orderbook" response = requests.get(url) orderbook_data = response.json() ​Response Structure The orderbook response contains two arrays of bids - one for YES positions and one for NO positions. Each bid is represented as a two-element array: [price, quantity]. ​Example Response Copy{ "orderbook": { "yes": [ [1, 200], // 200 contracts bid at 1¢ [15, 100], // 100 contracts bid at 15¢ [20, 50], // 50 contracts bid at 20¢ [25, 20], // 20 contracts bid at 25¢ [30, 11], // 11 contracts bid at 30¢ [31, 10], // 10 contracts bid at 31¢ [32, 10], // 10 contracts bid at 32¢ [33, 11], // 11 contracts bid at 33¢ [34, 9], // 9 contracts bid at 34¢ [35, 11], // 11 contracts bid at 35¢ [41, 10], // 10 contracts bid at 41¢ [42, 13] // 13 contracts bid at 42¢ ], "no": [ [1, 100], // 100 contracts bid at 1¢ [16, 3], // 3 contracts bid at 16¢ [25, 50], // 50 contracts bid at 25¢ [28, 19], // 19 contracts bid at 28¢ [36, 5], // 5 contracts bid at 36¢ [37, 50], // 50 contracts bid at 37¢ [38, 300], // 300 contracts bid at 38¢ [44, 29], // 29 contracts bid at 44¢ [45, 20], // 20 contracts bid at 45¢ [56, 17] // 17 contracts bid at 56¢ ] } ​Understanding the Arrays First element: Price in cents (1-99) Second element: Number of contracts available at that price Arrays are sorted by price in ascending order The highest bid (best bid) is the last element in each array ​Why Only Bids? Important: Kalshi’s orderbook only returns bids, not asks. This is because in binary prediction markets, there’s a reciprocal relationship between YES and NO positions. In binary prediction markets, every position has a complementary opposite: A YES BID at price X is equivalent to a NO ASK at price (100 - X) A NO BID at price Y is equivalent to a YES ASK at price (100 - Y) ​The Reciprocal Relationship Since binary markets must sum to 100¢, these relationships always hold: ActionEquivalent ToWhyYES BID at 60¢NO ASK at 40¢Willing to pay 60¢ for YES = Willing to receive 40¢ to take NONO BID at 30¢YES ASK at 70¢Willing to pay 30¢ for NO = Willing to receive 70¢ to take YES This reciprocal nature means that by showing only bids, the orderbook provides complete market information while avoiding redundancy. ​Calculating Spreads To find the bid-ask spread for a market: YES spread: Best YES bid: Highest price in the yes array Best YES ask: 100 - (Highest price in the no array) Spread = Best YES ask - Best YES bid NO spread: Best NO bid: Highest price in the no array Best NO ask: 100 - (Highest price in the yes array) Spread = Best NO ask - Best NO bid ​Example Calculation Copy# Using the example orderbook above best_yes_bid = 42 # Highest YES bid (last in array) best_yes_ask = 100 - 56 # 100 - highest NO bid = 44 spread = best_yes_ask - best_yes_bid # 44 - 42 = 2 # The spread is 2¢ # You can buy YES at 44¢ (implied ask) and sell at 42¢ (bid) ​Working with Orderbook Data ​Display Best Prices PythonJavaScriptCopydef display_best_prices(orderbook_data): """Display the best bid prices and implied asks""" orderbook = orderbook_data['orderbook'] # Best bids (if any exist) if orderbook['yes']: best_yes_bid = orderbook['yes'][-1][0] # Last element is highest print(f"Best YES Bid: {best_yes_bid}¢") if orderbook['no']: best_no_bid = orderbook['no'][-1][0] # Last element is highest best_yes_ask = 100 - best_no_bid print(f"Best YES Ask: {best_yes_ask}¢ (implied from NO bid)") print() if orderbook['no']: best_no_bid = orderbook['no'][-1][0] # Last element is highest print(f"Best NO Bid: {best_no_bid}¢") if orderbook['yes']: best_yes_bid = orderbook['yes'][-1][0] # Last element is highest best_no_ask = 100 - best_yes_bid print(f"Best NO Ask: {best_no_ask}¢ (implied from YES bid)") ​Calculate Market Depth Copydef calculate_depth(orderbook_data, depth_cents=5): """Calculate total volume within X cents of best bid""" orderbook = orderbook_data['orderbook'] yes_depth = 0 no_depth = 0 # YES side depth (iterate backwards from best bid) if orderbook['yes']: best_yes = orderbook['yes'][-1][0] # Last element is highest for price, quantity in reversed(orderbook['yes']): if best_yes - price <= depth_cents: yes_depth += quantity else: break # NO side depth (iterate backwards from best bid) if orderbook['no']: best_no = orderbook['no'][-1][0] # Last element is highest for price, quantity in reversed(orderbook['no']): if best_no - price <= depth_cents: no_depth += quantity else: break return {"yes_depth": yes_depth, "no_depth": no_depth} ​Next Steps Learn about making authenticated requests to place orders Explore WebSocket connections for real-time orderbook updates Read about market mechanics on the Kalshi website

## [​](\#getting-orderbook-data)  Getting Orderbook Data

The [Get Market Orderbook](/api-reference/market/get-market-order-book) endpoint returns the current state of bids for a specific market.

### [​](\#request-format)  Request Format

Copy

```
GET /markets/{ticker}/orderbook

```

No authentication is required for this endpoint.

### [​](\#example-request)  Example Request

Python

JavaScript

cURL

Copy

```
import requests

# Get orderbook for a specific market
market_ticker = "KXHIGHNY-24JAN01-T60"
url = f"https://api.elections.kalshi.com/trade-api/v2/markets/{market_ticker}/orderbook"

response = requests.get(url)
orderbook_data = response.json()

```

## [​](\#response-structure)  Response Structure

The orderbook response contains two arrays of bids - one for YES positions and one for NO positions. Each bid is represented as a two-element array: `[price, quantity]`.

### [​](\#example-response)  Example Response

Copy

```
{
  "orderbook": {
    "yes": [
      [1, 200],    // 200 contracts bid at 1¢
      [15, 100],   // 100 contracts bid at 15¢
      [20, 50],    // 50 contracts bid at 20¢
      [25, 20],    // 20 contracts bid at 25¢
      [30, 11],    // 11 contracts bid at 30¢
      [31, 10],    // 10 contracts bid at 31¢
      [32, 10],    // 10 contracts bid at 32¢
      [33, 11],    // 11 contracts bid at 33¢
      [34, 9],     // 9 contracts bid at 34¢
      [35, 11],    // 11 contracts bid at 35¢
      [41, 10],    // 10 contracts bid at 41¢
      [42, 13]     // 13 contracts bid at 42¢
    ],
    "no": [
      [1, 100],    // 100 contracts bid at 1¢
      [16, 3],     // 3 contracts bid at 16¢
      [25, 50],    // 50 contracts bid at 25¢
      [28, 19],    // 19 contracts bid at 28¢
      [36, 5],     // 5 contracts bid at 36¢
      [37, 50],    // 50 contracts bid at 37¢
      [38, 300],   // 300 contracts bid at 38¢
      [44, 29],    // 29 contracts bid at 44¢
      [45, 20],    // 20 contracts bid at 45¢
      [56, 17]     // 17 contracts bid at 56¢
    ]
  }
}

```

### [​](\#understanding-the-arrays)  Understanding the Arrays

- **First element**: Price in cents (1-99)
- **Second element**: Number of contracts available at that price
- Arrays are sorted by price in **ascending order**
- The **highest** bid (best bid) is the **last** element in each array

## [​](\#why-only-bids)  Why Only Bids?

**Important**: Kalshi’s orderbook only returns bids, not asks. This is because in binary prediction markets, there’s a reciprocal relationship between YES and NO positions.

In binary prediction markets, every position has a complementary opposite:

- A **YES BID** at price X is equivalent to a **NO ASK** at price (100 - X)
- A **NO BID** at price Y is equivalent to a **YES ASK** at price (100 - Y)

### [​](\#the-reciprocal-relationship)  The Reciprocal Relationship

Since binary markets must sum to 100¢, these relationships always hold:

| Action | Equivalent To | Why |
| --- | --- | --- |
| YES BID at 60¢ | NO ASK at 40¢ | Willing to pay 60¢ for YES = Willing to receive 40¢ to take NO |
| NO BID at 30¢ | YES ASK at 70¢ | Willing to pay 30¢ for NO = Willing to receive 70¢ to take YES |

This reciprocal nature means that by showing only bids, the orderbook provides complete market information while avoiding redundancy.

## [​](\#calculating-spreads)  Calculating Spreads

To find the bid-ask spread for a market:

1. **YES spread**:   - Best YES bid: Highest price in the `yes` array
   - Best YES ask: 100 - (Highest price in the `no` array)
   - Spread = Best YES ask - Best YES bid
2. **NO spread**:   - Best NO bid: Highest price in the `no` array
   - Best NO ask: 100 - (Highest price in the `yes` array)
   - Spread = Best NO ask - Best NO bid

### [​](\#example-calculation)  Example Calculation

Copy

```
# Using the example orderbook above
best_yes_bid = 42  # Highest YES bid (last in array)
best_yes_ask = 100 - 56  # 100 - highest NO bid = 44

spread = best_yes_ask - best_yes_bid  # 44 - 42 = 2

# The spread is 2¢
# You can buy YES at 44¢ (implied ask) and sell at 42¢ (bid)

```

## [​](\#working-with-orderbook-data)  Working with Orderbook Data

### [​](\#display-best-prices)  Display Best Prices

Python

JavaScript

Copy

```
def display_best_prices(orderbook_data):
    """Display the best bid prices and implied asks"""
    orderbook = orderbook_data['orderbook']

    # Best bids (if any exist)
    if orderbook['yes']:
        best_yes_bid = orderbook['yes'][-1][0]  # Last element is highest
        print(f"Best YES Bid: {best_yes_bid}¢")

    if orderbook['no']:
        best_no_bid = orderbook['no'][-1][0]  # Last element is highest
        best_yes_ask = 100 - best_no_bid
        print(f"Best YES Ask: {best_yes_ask}¢ (implied from NO bid)")

    print()

    if orderbook['no']:
        best_no_bid = orderbook['no'][-1][0]  # Last element is highest
        print(f"Best NO Bid: {best_no_bid}¢")

    if orderbook['yes']:
        best_yes_bid = orderbook['yes'][-1][0]  # Last element is highest
        best_no_ask = 100 - best_yes_bid
        print(f"Best NO Ask: {best_no_ask}¢ (implied from YES bid)")

```

### [​](\#calculate-market-depth)  Calculate Market Depth

Copy

```
def calculate_depth(orderbook_data, depth_cents=5):
    """Calculate total volume within X cents of best bid"""
    orderbook = orderbook_data['orderbook']

    yes_depth = 0
    no_depth = 0

    # YES side depth (iterate backwards from best bid)
    if orderbook['yes']:
        best_yes = orderbook['yes'][-1][0]  # Last element is highest
        for price, quantity in reversed(orderbook['yes']):
            if best_yes - price <= depth_cents:
                yes_depth += quantity
            else:
                break

    # NO side depth (iterate backwards from best bid)
    if orderbook['no']:
        best_no = orderbook['no'][-1][0]  # Last element is highest
        for price, quantity in reversed(orderbook['no']):
            if best_no - price <= depth_cents:
                no_depth += quantity
            else:
                break

    return {"yes_depth": yes_depth, "no_depth": no_depth}

```

## [​](\#next-steps)  Next Steps

- Learn about [making authenticated requests](/getting_started/api_keys) to place orders
- Explore [WebSocket connections](/websockets/orderbook-updates) for real-time orderbook updates
- Read about [market mechanics](https://kalshi.com/learn) on the Kalshi website


---

###### Quick Start: WebSockets - API DocumentationQuick Start: WebSockets​Overview Kalshi’s WebSocket API provides real-time updates for: Order book changes Trade executions Market status updates Fill notifications (authenticated connections only) ​Connection URL Connect to the WebSocket endpoint at: Copywss://api.elections.kalshi.com/trade-api/ws/v2 For the demo environment, use: Copywss://demo-api.kalshi.co/trade-api/ws/v2 ​Authentication WebSocket connections require authentication using the same API key signing mechanism as REST endpoints. For detailed information about API key generation and request signing, see our API Keys documentation. ​Required Headers When establishing the WebSocket connection, include these headers: CopyKALSHI-ACCESS-KEY: your_api_key_id KALSHI-ACCESS-SIGNATURE: request_signature KALSHI-ACCESS-TIMESTAMP: unix_timestamp_in_milliseconds ​Signing the WebSocket Request The signature for WebSocket connections follows the same pattern as REST API requests: Create the message to sign: Copytimestamp + "GET" + "/trade-api/ws/v2" Generate the signature using your private key (see API Keys documentation) Include the headers when opening the WebSocket connection ​Establishing a Connection To connect to the WebSocket API, you need to: Generate authentication headers (same as REST API) Create a WebSocket connection with those headers Handle the connection lifecycle Here’s how to establish an authenticated connection: Copyimport websockets import asyncio # WebSocket URL ws_url = "wss://demo-api.kalshi.co/trade-api/ws/v2" # Demo environment # Generate authentication headers (see API Keys documentation) auth_headers = { "KALSHI-ACCESS-KEY": "your_api_key_id", "KALSHI-ACCESS-SIGNATURE": "generated_signature", "KALSHI-ACCESS-TIMESTAMP": "timestamp_in_milliseconds" } # Connect with authentication async def connect(): async with websockets.connect(ws_url, additional_headers=auth_headers) as websocket: print("Connected to Kalshi WebSocket") # Connection is now established # You can start sending and receiving messages # Listen for messages async for message in websocket: print(f"Received: {message}") # Run the connection asyncio.run(connect()) ​Subscribing to Data Once connected, subscribe to channels by sending a subscription command: Copyimport json async def subscribe_to_ticker(websocket): """Subscribe to ticker updates""" subscription = { "id": 1, "cmd": "subscribe", "params": { "channels": ["ticker"] } await websocket.send(json.dumps(subscription)) async def subscribe_to_orderbook(websocket, market_tickers): """Subscribe to orderbook updates for specific markets""" subscription = { "id": 2, "cmd": "subscribe", "params": { "channels": ["orderbook_delta"], "market_tickers": market_tickers } await websocket.send(json.dumps(subscription)) ​Processing Messages Handle incoming messages based on their type: Copyasync def process_message(message): """Process incoming WebSocket messages""" data = json.loads(message) msg_type = data.get("type") if msg_type == "ticker": # Handle ticker update market = data["data"]["market_ticker"] bid = data["data"]["bid"] ask = data["data"]["ask"] print(f"{market}: Bid ${bid}, Ask ${ask}") elif msg_type == "orderbook_snapshot": # Handle full orderbook state print(f"Orderbook snapshot for {data['data']['market_ticker']}") elif msg_type == "orderbook_update": # Handle orderbook changes print(f"Orderbook update for {data['data']['market_ticker']}") # Note: client_order_id field is optional - present only when you caused this change if 'client_order_id' in data['data']: print(f" Your order {data['data']['client_order_id']} caused this change") elif msg_type == "error": error_code = data.get("msg", {}).get("code") error_msg = data.get("msg", {}).get("msg") print(f"Error {error_code}: {error_msg}") ​Connection Keep-Alive The Python websockets library automatically handles WebSocket ping/pong frames to keep connections alive. No manual heartbeat handling is required. Learn more about automatic keepalive in the websockets documentation.Other WebSocket libraries may require manual ping/pong implementation. ​Subscribing to Channels Once connected, subscribe to specific data channels: ​Subscribe to Ticker Updates To receive real-time ticker updates for all markets: Copyasync def subscribe_to_tickers(self): """Subscribe to ticker updates for all markets""" subscription_message = { "id": self.message_id, "cmd": "subscribe", "params": { "channels": ["ticker"] } await self.ws.send(json.dumps(subscription_message)) self.message_id += 1 ​Subscribe to Specific Markets To subscribe to orderbook or trade updates for specific markets: Copyasync def subscribe_to_markets(self, channels, market_tickers): """Subscribe to specific channels and markets""" subscription_message = { "id": self.message_id, "cmd": "subscribe", "params": { "channels": channels, "market_tickers": market_tickers } await self.ws.send(json.dumps(subscription_message)) self.message_id += 1 # Example usage: # Subscribe to orderbook updates await subscribe_to_markets(["orderbook"], ["KXFUT24-LSV", "KXHARRIS24-LSV"]) # Subscribe to trade feed await subscribe_to_markets(["trades"], ["KXFUT24-LSV"]) ​Connection Lifecycle Initial Connection: Establish WebSocket with authentication headers Subscribe: Send subscription commands for desired channels Receive Updates: Process incoming messages based on their type Handle Disconnects: Implement reconnection logic with exponential backoff ​Error Handling The server sends error messages in this format: Copy{ "id": 123, "type": "error", "msg": { "code": 6, "msg": "Params required" } ​WebSocket Error Codes CodeErrorDescription1Unable to process messageGeneral processing error2Params requiredMissing params object in command3Channels requiredMissing channels array in subscribe4Subscription IDs requiredMissing sids in unsubscribe5Unknown commandInvalid command name7Unknown subscription IDSubscription ID not found8Unknown channel nameInvalid channel in subscribe9Authentication requiredPrivate channel without auth10Channel errorChannel-specific error11Invalid parameterMalformed parameter value12Exactly one subscription ID requiredFor update_subscription13Unsupported actionInvalid action for update_subscription14Market ticker requiredMissing market specification15Action requiredMissing action in update_subscription16Market not foundInvalid market ticker17Internal errorServer-side processing error ​Best Practices Connection Management Implement automatic reconnection with exponential backoff Handle network interruptions gracefully Use the websockets library’s built-in keepalive Data Handling Process messages asynchronously to avoid blocking Implement proper error handling for malformed messages Cache initial orderbook state before applying updates Security Never expose your private key in client-side code Rotate API keys regularly Use secure key storage practices Performance Subscribe only to markets you need Implement message buffering for high-frequency updates Consider using connection pooling for multiple subscriptions ​Complete Example Here’s a complete, runnable example that connects to the WebSocket API and subscribes to orderbook updates: Copyimport asyncio import base64 import json import time import websockets from cryptography.hazmat.primitives import serialization, hashes from cryptography.hazmat.primitives.asymmetric import padding # Configuration KEY_ID = "your_api_key_id" PRIVATE_KEY_PATH = "path/to/private_key.pem" MARKET_TICKER = "KXHARRIS24-LSV" # Replace with any open market WS_URL = "wss://demo-api.kalshi.co/trade-api/ws/v2" def sign_pss_text(private_key, text: str) -> str: """Sign message using RSA-PSS""" message = text.encode('utf-8') signature = private_key.sign( message, padding.PSS( mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.DIGEST_LENGTH ), hashes.SHA256() ) return base64.b64encode(signature).decode('utf-8') def create_headers(private_key, method: str, path: str) -> dict: """Create authentication headers""" timestamp = str(int(time.time() * 1000)) msg_string = timestamp + method + path.split('?')[0] signature = sign_pss_text(private_key, msg_string) return { "Content-Type": "application/json", "KALSHI-ACCESS-KEY": KEY_ID, "KALSHI-ACCESS-SIGNATURE": signature, "KALSHI-ACCESS-TIMESTAMP": timestamp, } async def orderbook_websocket(): """Connect to WebSocket and subscribe to orderbook""" # Load private key with open(PRIVATE_KEY_PATH, 'rb') as f: private_key = serialization.load_pem_private_key( f.read(), password=None ) # Create WebSocket headers ws_headers = create_headers(private_key, "GET", "/trade-api/ws/v2") async with websockets.connect(WS_URL, additional_headers=ws_headers) as websocket: print(f"Connected! Subscribing to orderbook for {MARKET_TICKER}") # Subscribe to orderbook subscribe_msg = { "id": 1, "cmd": "subscribe", "params": { "channels": ["orderbook_delta"], "market_ticker": MARKET_TICKER } await websocket.send(json.dumps(subscribe_msg)) # Process messages async for message in websocket: data = json.loads(message) msg_type = data.get("type") if msg_type == "subscribed": print(f"Subscribed: {data}") elif msg_type == "orderbook_snapshot": print(f"Orderbook snapshot: {data}") elif msg_type == "orderbook_delta": # The client_order_id field is optional - only present when you caused the change if 'client_order_id' in data.get('data', {}): print(f"Orderbook update (your order {data['data']['client_order_id']}): {data}") else: print(f"Orderbook update: {data}") elif msg_type == "error": print(f"Error: {data}") # Run the example if __name__ == "__main__": asyncio.run(orderbook_websocket()) This example: Establishes an authenticated WebSocket connection Subscribes to orderbook updates for the specified market Processes both the initial snapshot and incremental updates Displays orderbook changes in real-time To run this example: Replace KEY_ID with your API key ID Replace PRIVATE_KEY_PATH with the path to your private key file Replace MARKET_TICKER with any open market ticker Run with Python 3.7+ ​Next Steps Review the WebSocket API Reference for detailed message specifications Explore Market Data Quick Start for REST API integration Check out our Demo Environment for testing

## [​](\#overview)  Overview

Kalshi’s WebSocket API provides real-time updates for:

- Order book changes
- Trade executions
- Market status updates
- Fill notifications (authenticated connections only)

## [​](\#connection-url)  Connection URL

Connect to the WebSocket endpoint at:

Copy

```
wss://api.elections.kalshi.com/trade-api/ws/v2

```

For the demo environment, use:

Copy

```
wss://demo-api.kalshi.co/trade-api/ws/v2

```

## [​](\#authentication)  Authentication

WebSocket connections require authentication using the same API key signing mechanism as REST endpoints.

For detailed information about API key generation and request signing, see our [API Keys documentation](/getting_started/api_keys).

### [​](\#required-headers)  Required Headers

When establishing the WebSocket connection, include these headers:

Copy

```
KALSHI-ACCESS-KEY: your_api_key_id
KALSHI-ACCESS-SIGNATURE: request_signature
KALSHI-ACCESS-TIMESTAMP: unix_timestamp_in_milliseconds

```

### [​](\#signing-the-websocket-request)  Signing the WebSocket Request

The signature for WebSocket connections follows the same pattern as REST API requests:

1. **Create the message to sign:**

   Copy

   ```
   timestamp + "GET" + "/trade-api/ws/v2"

   ```

2. **Generate the signature** using your private key (see [API Keys documentation](/getting_started/api_keys#signing-requests))
3. **Include the headers** when opening the WebSocket connection

## [​](\#establishing-a-connection)  Establishing a Connection

To connect to the WebSocket API, you need to:

1. Generate authentication headers (same as REST API)
2. Create a WebSocket connection with those headers
3. Handle the connection lifecycle

Here’s how to establish an authenticated connection:

Copy

```
import websockets
import asyncio

# WebSocket URL
ws_url = "wss://demo-api.kalshi.co/trade-api/ws/v2"  # Demo environment

# Generate authentication headers (see API Keys documentation)
auth_headers = {
    "KALSHI-ACCESS-KEY": "your_api_key_id",
    "KALSHI-ACCESS-SIGNATURE": "generated_signature",
    "KALSHI-ACCESS-TIMESTAMP": "timestamp_in_milliseconds"
}

# Connect with authentication
async def connect():
    async with websockets.connect(ws_url, additional_headers=auth_headers) as websocket:
        print("Connected to Kalshi WebSocket")

        # Connection is now established
        # You can start sending and receiving messages

        # Listen for messages
        async for message in websocket:
            print(f"Received: {message}")

# Run the connection
asyncio.run(connect())

```

## [​](\#subscribing-to-data)  Subscribing to Data

Once connected, subscribe to channels by sending a subscription command:

Copy

```
import json

async def subscribe_to_ticker(websocket):
    """Subscribe to ticker updates"""
    subscription = {
        "id": 1,
        "cmd": "subscribe",
        "params": {
            "channels": ["ticker"]
        }
    }
    await websocket.send(json.dumps(subscription))

async def subscribe_to_orderbook(websocket, market_tickers):
    """Subscribe to orderbook updates for specific markets"""
    subscription = {
        "id": 2,
        "cmd": "subscribe",
        "params": {
            "channels": ["orderbook_delta"],
            "market_tickers": market_tickers
        }
    }
    await websocket.send(json.dumps(subscription))

```

## [​](\#processing-messages)  Processing Messages

Handle incoming messages based on their type:

Copy

```
async def process_message(message):
    """Process incoming WebSocket messages"""
    data = json.loads(message)
    msg_type = data.get("type")

    if msg_type == "ticker":
        # Handle ticker update
        market = data["data"]["market_ticker"]
        bid = data["data"]["bid"]
        ask = data["data"]["ask"]
        print(f"{market}: Bid ${bid}, Ask ${ask}")

    elif msg_type == "orderbook_snapshot":
        # Handle full orderbook state
        print(f"Orderbook snapshot for {data['data']['market_ticker']}")

    elif msg_type == "orderbook_update":
        # Handle orderbook changes
        print(f"Orderbook update for {data['data']['market_ticker']}")
        # Note: client_order_id field is optional - present only when you caused this change
        if 'client_order_id' in data['data']:
            print(f"  Your order {data['data']['client_order_id']} caused this change")

    elif msg_type == "error":
        error_code = data.get("msg", {}).get("code")
        error_msg = data.get("msg", {}).get("msg")
        print(f"Error {error_code}: {error_msg}")

```

## [​](\#connection-keep-alive)  Connection Keep-Alive

The Python `websockets` library automatically handles WebSocket ping/pong frames to keep connections alive. No manual heartbeat handling is required. Learn more about [automatic keepalive in the websockets documentation](https://websockets.readthedocs.io/en/stable/topics/design.html#keepalive).Other WebSocket libraries may require manual ping/pong implementation.

## [​](\#subscribing-to-channels)  Subscribing to Channels

Once connected, subscribe to specific data channels:

### [​](\#subscribe-to-ticker-updates)  Subscribe to Ticker Updates

To receive real-time ticker updates for all markets:

Copy

```
async def subscribe_to_tickers(self):
    """Subscribe to ticker updates for all markets"""
    subscription_message = {
        "id": self.message_id,
        "cmd": "subscribe",
        "params": {
            "channels": ["ticker"]
        }
    }
    await self.ws.send(json.dumps(subscription_message))
    self.message_id += 1

```

### [​](\#subscribe-to-specific-markets)  Subscribe to Specific Markets

To subscribe to orderbook or trade updates for specific markets:

Copy

```
async def subscribe_to_markets(self, channels, market_tickers):
    """Subscribe to specific channels and markets"""
    subscription_message = {
        "id": self.message_id,
        "cmd": "subscribe",
        "params": {
            "channels": channels,
            "market_tickers": market_tickers
        }
    }
    await self.ws.send(json.dumps(subscription_message))
    self.message_id += 1

# Example usage:
# Subscribe to orderbook updates
await subscribe_to_markets(["orderbook"], ["KXFUT24-LSV", "KXHARRIS24-LSV"])

# Subscribe to trade feed
await subscribe_to_markets(["trades"], ["KXFUT24-LSV"])

```

## [​](\#connection-lifecycle)  Connection Lifecycle

1. **Initial Connection**: Establish WebSocket with authentication headers
2. **Subscribe**: Send subscription commands for desired channels
3. **Receive Updates**: Process incoming messages based on their type
4. **Handle Disconnects**: Implement reconnection logic with exponential backoff

## [​](\#error-handling)  Error Handling

The server sends error messages in this format:

Copy

```
{
  "id": 123,
  "type": "error",
  "msg": {
    "code": 6,
    "msg": "Params required"
  }
}

```

### [​](\#websocket-error-codes)  WebSocket Error Codes

| Code | Error | Description |
| --- | --- | --- |
| 1 | Unable to process message | General processing error |
| 2 | Params required | Missing params object in command |
| 3 | Channels required | Missing channels array in subscribe |
| 4 | Subscription IDs required | Missing sids in unsubscribe |
| 5 | Unknown command | Invalid command name |
| 7 | Unknown subscription ID | Subscription ID not found |
| 8 | Unknown channel name | Invalid channel in subscribe |
| 9 | Authentication required | Private channel without auth |
| 10 | Channel error | Channel-specific error |
| 11 | Invalid parameter | Malformed parameter value |
| 12 | Exactly one subscription ID required | For update\_subscription |
| 13 | Unsupported action | Invalid action for update\_subscription |
| 14 | Market ticker required | Missing market specification |
| 15 | Action required | Missing action in update\_subscription |
| 16 | Market not found | Invalid market ticker |
| 17 | Internal error | Server-side processing error |

## [​](\#best-practices)  Best Practices

## Connection Management

- Implement automatic reconnection with exponential backoff
- Handle network interruptions gracefully
- Use the websockets library’s built-in keepalive

## Data Handling

- Process messages asynchronously to avoid blocking
- Implement proper error handling for malformed messages
- Cache initial orderbook state before applying updates

## Security

- Never expose your private key in client-side code
- Rotate API keys regularly
- Use secure key storage practices

## Performance

- Subscribe only to markets you need
- Implement message buffering for high-frequency updates
- Consider using connection pooling for multiple subscriptions

## [​](\#complete-example)  Complete Example

Here’s a complete, runnable example that connects to the WebSocket API and subscribes to orderbook updates:

Copy

```
import asyncio
import base64
import json
import time
import websockets
from cryptography.hazmat.primitives import serialization, hashes
from cryptography.hazmat.primitives.asymmetric import padding

# Configuration
KEY_ID = "your_api_key_id"
PRIVATE_KEY_PATH = "path/to/private_key.pem"
MARKET_TICKER = "KXHARRIS24-LSV"  # Replace with any open market
WS_URL = "wss://demo-api.kalshi.co/trade-api/ws/v2"

def sign_pss_text(private_key, text: str) -> str:
    """Sign message using RSA-PSS"""
    message = text.encode('utf-8')
    signature = private_key.sign(
        message,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.DIGEST_LENGTH
        ),
        hashes.SHA256()
    )
    return base64.b64encode(signature).decode('utf-8')

def create_headers(private_key, method: str, path: str) -> dict:
    """Create authentication headers"""
    timestamp = str(int(time.time() * 1000))
    msg_string = timestamp + method + path.split('?')[0]
    signature = sign_pss_text(private_key, msg_string)

    return {
        "Content-Type": "application/json",
        "KALSHI-ACCESS-KEY": KEY_ID,
        "KALSHI-ACCESS-SIGNATURE": signature,
        "KALSHI-ACCESS-TIMESTAMP": timestamp,
    }

async def orderbook_websocket():
    """Connect to WebSocket and subscribe to orderbook"""
    # Load private key
    with open(PRIVATE_KEY_PATH, 'rb') as f:
        private_key = serialization.load_pem_private_key(
            f.read(),
            password=None
        )

    # Create WebSocket headers
    ws_headers = create_headers(private_key, "GET", "/trade-api/ws/v2")

    async with websockets.connect(WS_URL, additional_headers=ws_headers) as websocket:
        print(f"Connected! Subscribing to orderbook for {MARKET_TICKER}")

        # Subscribe to orderbook
        subscribe_msg = {
            "id": 1,
            "cmd": "subscribe",
            "params": {
                "channels": ["orderbook_delta"],
                "market_ticker": MARKET_TICKER
            }
        }
        await websocket.send(json.dumps(subscribe_msg))

        # Process messages
        async for message in websocket:
            data = json.loads(message)
            msg_type = data.get("type")

            if msg_type == "subscribed":
                print(f"Subscribed: {data}")

            elif msg_type == "orderbook_snapshot":
                print(f"Orderbook snapshot: {data}")

            elif msg_type == "orderbook_delta":
                # The client_order_id field is optional - only present when you caused the change
                if 'client_order_id' in data.get('data', {}):
                    print(f"Orderbook update (your order {data['data']['client_order_id']}): {data}")
                else:
                    print(f"Orderbook update: {data}")

            elif msg_type == "error":
                print(f"Error: {data}")

# Run the example
if __name__ == "__main__":
    asyncio.run(orderbook_websocket())

```

This example:

- Establishes an authenticated WebSocket connection
- Subscribes to orderbook updates for the specified market
- Processes both the initial snapshot and incremental updates
- Displays orderbook changes in real-time

To run this example:

1. Replace `KEY_ID` with your API key ID
2. Replace `PRIVATE_KEY_PATH` with the path to your private key file
3. Replace `MARKET_TICKER` with any open market ticker
4. Run with Python 3.7+

## [​](\#next-steps)  Next Steps

- Review the [WebSocket API Reference](/websockets) for detailed message specifications
- Explore [Market Data Quick Start](/getting_started/quick_start_market_data) for REST API integration
- Check out our [Demo Environment](/getting_started/demo_env) for testing


---

###### Rate Limits and Tiers - API DocumentationRate Limits and Tiers​Access tiers TierReadWriteBasic20 per second10 per secondAdvanced30 per second30 per secondPremier100 per second100 per secondPrime400 per second400 per second Qualification for tiers: Basic: Completing signup Advanced: Completing https://kalshi.typeform.com/advanced-api Premier: 3.75% of exchange traded volume in a given month Prime: 7.5% of exchange traded volume in a given month In addition to the volume targets, technical competency is a requirement for Premier/Prime access. Before providing access to the Premier/Prime tiers, the Exchange will establish that the trader/trading entity has the following requirements met: Knowledge of common security practices for API usage Proficiency in setting up monitoring for API usage, and ability to monitor API usage in near real-time Understanding and implementation of rate limiting and throttling mechanisms imposed by the API, and the ability to self-limit load Awareness of legal and compliance aspects related to API usage Only the following APIs fall under the write limit, for the batch APIs, each item in the batch is considered 1 transaction with the sole exception of BatchCancelOrders, where each cancel counts as 0.2 transactions: BatchCreateOrders BatchCancelOrders CreateOrder CancelOrder AmendOrder DecreaseOrder We reserve the right to downgrade your API rate limit tier from Prime and Premier when you have shown lack of activity in the previous period. At any time, any Member that uses FIX or is at the highest possible API tier is eligible for an upgrade to its rate limit upon demonstration that such a tier is necessary for its bona fide market activity.

## [​](\#access-tiers)  Access tiers

| Tier | Read | Write |
| --- | --- | --- |
| Basic | 20 per second | 10 per second |
| Advanced | 30 per second | 30 per second |
| Premier | 100 per second | 100 per second |
| Prime | 400 per second | 400 per second |

Qualification for tiers:

- Basic: Completing signup
- Advanced: Completing [https://kalshi.typeform.com/advanced-api](https://kalshi.typeform.com/advanced-api)
- Premier: 3.75% of exchange traded volume in a given month
- Prime: 7.5% of exchange traded volume in a given month

In addition to the volume targets, technical competency is a requirement for Premier/Prime access. Before providing access to the Premier/Prime tiers, the Exchange will establish that the trader/trading entity has the following requirements met:

- Knowledge of common security practices for API usage
- Proficiency in setting up monitoring for API usage, and ability to monitor API usage in near real-time
- Understanding and implementation of rate limiting and throttling mechanisms imposed by the API, and the ability to self-limit load
- Awareness of legal and compliance aspects related to API usage

Only the following APIs fall under the write limit, for the batch APIs, each item in the batch is considered 1 transaction with the sole exception of BatchCancelOrders, where each cancel counts as 0.2 transactions:

- [BatchCreateOrders](/api-reference/portfolio/batch-create-orders)
- [BatchCancelOrders](/api-reference/portfolio/batch-cancel-orders)
- [CreateOrder](/api-reference/portfolio/create-order)
- [CancelOrder](/api-reference/portfolio/cancel-order)
- [AmendOrder](/api-reference/portfolio/amend-order)
- [DecreaseOrder](/api-reference/portfolio/decrease-order)

We reserve the right to downgrade your API rate limit tier from Prime and Premier when you have shown lack of activity in the previous period.

At any time, any Member that uses FIX or is at the highest possible API tier is eligible for an upgrade to its rate limit upon demonstration that such a tier is necessary for its bona fide market activity.


---

###### Subpenny Pricing - API DocumentationSubpenny Pricing​Format Copy{ "price": 12, // legacy: cents "price_dollars": "0.1200" // new: fixed-point dollars } Starting soon in the API, you will begin to see prices and money represented in 2 different formats: integer cents (legacy) and fixed-point dollars (new). A fixed-point dollar is a string bearing a fixed-point representation of money accurate to at least 4 decimal points. ​Motivation Subpenny pricing will allow for more accurate pricing and the tail end of markets where likelihood of a given event are close to 100% or 0%. ​Status Currently the minimum tick size on all markets is still 1 cent. Additionally, all prices and money fields will continue to be available in the legacy integer cents format. However, in the near future we will be introducing sub-penny pricing on orders. As such, we will eventually the legacy integer cents format. Therefore, please update systems to parse the new fixed-point dollars fields and prepare for subpenny precision.

## [​](\#format)  Format

Copy

```
{
    "price": 12,              // legacy: cents
    "price_dollars": "0.1200" // new: fixed-point dollars
}

```

Starting soon in the API, you will begin to see prices and money represented in 2 different formats: integer cents (legacy) and fixed-point dollars (new).
A fixed-point dollar is a string bearing a fixed-point representation of money accurate to at least 4 decimal points.

## [​](\#motivation)  Motivation

Subpenny pricing will allow for more accurate pricing and the tail end of markets where likelihood of a given event are close to 100% or 0%.

## [​](\#status)  Status

Currently the minimum tick size on all markets is still 1 cent.
Additionally, all prices and money fields will continue to be available in the legacy integer cents format.However, in the near future we will be introducing sub-penny pricing on orders. As such, we will eventually the legacy integer cents format.
Therefore, please update systems to parse the new fixed-point dollars fields and prepare for subpenny precision.
