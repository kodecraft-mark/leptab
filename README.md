# leptab
Leptos data table using tailwind as style

## Additional details
This projects is still under development.
If you are using Leptos with tailwind, it can be a perfect match for your data table UI.

## Features
- Sort
- Download
- Search
- Row per page control
- Pagination

## Example
```rust
#[component]
#[allow(non_snake_case)]
pub fn ExampleTable() -> impl IntoView {
	//Create headers
    let headers = RwSignal::new(vec![
        TableHeader::new("user_id", "user_id", "Id", false, "", false, "", "", "", false),
        TableHeader::new("name", "name", "Name", false, "", false, "", "", "", false),
        TableHeader::new("profit", "profit_amount", "Profit Amount", true, "profit_currency", true, "- -", "", "", false),
        TableHeader::new("current_status", "current_status", "Status", false, "", false, "No movement", "Gain", "Loss", false),
    ]);
	//Signal for sort, ascending is true, descending is false
    let sort = RwSignal::new(false);
	
	//Signal for sort_by (name of the column to sort)
    let sort_by = RwSignal::new("".to_string());
	
	//Offset, the start of the row to show
    let offset = RwSignal::new(0u32);
	
	//Limit, the end index of row to show
    let limit = RwSignal::new(5u32);
	
	//Search, keyword for search
    let search = RwSignal::new("".to_string());
	
	//Filter, a filter string for your query
    let filter = RwSignal::new("set some query filter here".to_string());
    let fields = RwSignal::new("set some fields here".to_string());
	
	//Create a resource for data count, make sure that search is a dependency (dependes on your implementation)
    let data_count_resource = create_local_resource(move || (search.get()), move |a| get_collection_count(filter.get(), Some(a)));
	
	//Get collection here, make the necessary signals as dependency (dependes on your implementation)
    let data_resource = create_local_resource(move || (filter.get(), fields.get(), offset.get(), limit.get(), search.get(), sort.get(), sort_by.get()),
        move |(a,b,c,d,e,f ,g)| get_collection::<UserProfitResponse>(a, b, c, Some(d), Some(e), Some(f), Some(g)));
	
	//Data count, it will hold the current count
    let data_count = RwSignal::new(0u32);
	
	//Some extraction (not necessary, depends on you implementation)
    let extracted_data = RwSignal::new(Vec::<ExtractedUserProfit>::default());
	
	//Convert to json data
    let json_data = Signal::derive(move || {
        extracted_data
            .get()
            .into_iter()
            .map(serde_json::to_value)
            .collect::<Result<Vec<serde_json::Value>, _>>()
            .expect("Failed to serialize to JSON")
    });
	//If you want a download then add it here
    let download_data_request = Signal::derive(move || {
        DownloadDataRequest {
            table_name: "trade".to_string(),
            filter: filter.get(),
            fields: fields.get(),
            search: search.get(),
        }
    });
    //Create a download resource, with DownloadDataRequest as request parameter
    let download_resource: Resource<DownloadDataRequest, Result<String, ServerFnError>> = create_local_resource(move || (download_data_request.get()), move |d| get_collection_file::<UserProfitResponse>(d));
    let allow_download = RwSignal::new(true);
    let download_filename = RwSignal::new("user_profit_file".to_string());
	
	//Current page number
    let current_page = RwSignal::new(1u32);
    view! {
        <div>
            <Transition 
                fallback = move || view! {
                    <div class = "items-center">
                        <div class = "flex justify-center h-screen">
							//Add some loading spinners
                        </div>
                    </div>
                }
            >
                {
                    move || {
                        data_count_resource.and_then(|c| {
                            data_count.set(*c);
                            data_resource.and_then(|d| {
                                extracted_data.set(d.extract());
                                view! {
                                    <DataTable 
                                        headers = headers 
                                        data = json_data 
                                        offset = offset 
                                        search = search 
                                        sort = sort 
                                        sort_by = sort_by 
                                        limit = limit 
                                        total = data_count 
                                        current_page = current_page
                                        allow_download = allow_download
                                        download_filename = download_filename
                                        download_resource = download_resource
                                    />
                                }
                            })
                        })
                    }
                }
            </Transition>
        </div>
    }
}
```
