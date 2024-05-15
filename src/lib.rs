
pub mod model;
use model::*;
use leptos::*;
use serde_json::Value;


/// Data table component
/// 
/// # Arguments
/// 
/// * `headers` - headers with extra data for the table
/// * `data` - data to display in the table
/// * `offset` - data offset for pagination
/// * `search` - search string
/// * `sort` - sort ascending (fdlse), descending (true)
/// * `sort_by` - sort by column name
/// * `limit` - number of rows to display per page
/// * `total` - total number of rows
/// * `current_page` - current page number
/// * `download` - download data
#[allow(non_snake_case)]
#[component]
pub fn DataTable(
    headers: RwSignal<Vec<TableHeader>>,
    data: Signal<Vec<Value>>,
    offset: RwSignal<u32>,
    search: RwSignal<String>,
    sort: RwSignal<bool>,
    sort_by: RwSignal<String>,
    limit: RwSignal<u32>,
    total: RwSignal<u32>,
    current_page: RwSignal<u32>,
    download: RwSignal<DataDownload>,
) -> impl IntoView {
    let pages_entries = RwSignal::new(vec![5, 10, 15, 20, 25, 50, 100]);
    view! {
        <div class="p-1">
            <div class="flex justify-between">
                <div class="flex flex-auto justify-start gap-1">
                    <select
                        class="text-xs border-gray-800 rounded shadow-md select-sm hover:shadow-sm hover:shadow-success bg-base-100"
                        name="row_slice"
                        on:change=move |e| {
                            let val = event_target_value(&e).parse::<u32>().unwrap();
                            limit.set(val);
                        }
                    >

                        {move || {
                            pages_entries
                                .get()
                                .into_iter()
                                .map(|page_entry| {
                                    view! {
                                        <option
                                            prop:selected=limit.get() == page_entry
                                            value=page_entry.to_string()
                                        >
                                            {page_entry}
                                        </option>
                                    }
                                })
                                .collect_view()
                        }}

                    </select>

                    {move || {
                        match download.get().is_allowed {
                            true => {
                                view! {
                                    <DownloadCsvAnchor
                                        content=download.get().file_content
                                        file_name=download.get().file_name
                                    />
                                }
                            }
                            false => view! {}.into_view(),
                        }
                    }}

                </div>
                <div class="flex flex-auto justify-end gap-1 join">
                    <div>
                        <span class="text-sm font-light">Search:</span>
                        <input
                            type="text"
                            class="input input-sm input-info focus:outline-none focus:shadow-outline "
                            placeholder=""
                            prop:value=search
                            on:blur=move |event| {
                                search.set(event_target_value(&event));
                                current_page.set(1);
                                offset.set(0);
                            }
                        />

                    </div>
                </div>
            </div>
            <table class="table table-xs table-zebra-zebra mt-1">
                <thead>
                    <tr>

                        {move || {
                            headers
                                .get()
                                .into_iter()
                                .map(|i| {
                                    let header = RwSignal::new(i);
                                    view! { <TableHeader sort_by=sort_by header=header sort=sort/> }
                                })
                                .collect_view()
                        }}

                    </tr>
                </thead>
                <tbody>
                    {move || {
                        match data.get().is_empty() {
                            true => {
                                view! {
                                    <tr>
                                        <td colspan=headers.get().len() class="text-center">
                                            <span class="opacity-50 font-extralight">
                                                No data available
                                            </span>
                                        </td>
                                    </tr>
                                }
                                    .into_view()
                            }
                            false => {
                                view! {
                                    {move || {
                                        {
                                            data.get()
                                                .into_iter()
                                                .map(|value| {
                                                    view! {
                                                        <tr>

                                                            {move || {
                                                                headers
                                                                    .get()
                                                                    .into_iter()
                                                                    .map(|header| {
                                                                        view! {
                                                                            <td>

                                                                                {
                                                                                    let number_style = match header
                                                                                        .find(&value)
                                                                                        .parse::<f64>()
                                                                                        .ok()
                                                                                    {
                                                                                        Some(parsed_value) if parsed_value >= 0.0 => "text-success",
                                                                                        Some(_) => "text-error",
                                                                                        None => "",
                                                                                    };
                                                                                    let style_when_success = match header
                                                                                        .find(&value)
                                                                                        .to_uppercase() == header.style_when_success.to_uppercase()
                                                                                    {
                                                                                        true => "text-success",
                                                                                        false => "",
                                                                                    };
                                                                                    let style_when_error = match header
                                                                                        .find(&value)
                                                                                        .to_uppercase() == header.style_when_error.to_uppercase()
                                                                                    {
                                                                                        true => "text-error",
                                                                                        false => "",
                                                                                    };
                                                                                    let case_style = match header.to_uppercase {
                                                                                        true => "uppercase",
                                                                                        false => "",
                                                                                    };
                                                                                    let style = format!(
                                                                                        "{} {} {} {}",
                                                                                        number_style,
                                                                                        style_when_success,
                                                                                        style_when_error,
                                                                                        case_style,
                                                                                    );
                                                                                    view! { <span class=style>{header.find(&value)}</span> }
                                                                                }
                                                                                {match header.is_currency {
                                                                                    true => {
                                                                                        view! {
                                                                                            <span class="text-xs opacity-50">
                                                                                                {format!(" {}", header.find_currency(&value))}
                                                                                            </span>
                                                                                        }
                                                                                    }
                                                                                    false => view! { <span></span> },
                                                                                }}

                                                                            </td>
                                                                        }
                                                                    })
                                                                    .collect_view()
                                                            }}

                                                        </tr>
                                                    }
                                                })
                                                .collect_view()
                                        }
                                            .into_view()
                                    }}
                                }
                                    .into_view()
                            }
                        }
                    }}

                </tbody>
                <tfoot>
                    <tr>
                        <td colspan=move || headers.get().len()>
                            <TablePagination
                                total=total
                                limit=limit
                                offset=offset
                                current_page=current_page
                            />
                        </td>
                    </tr>
                </tfoot>
            </table>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn TableHeader(sort_by: RwSignal<String>, header: RwSignal<TableHeader>, sort: RwSignal<bool>) -> impl IntoView {
    view! {
        <th
            class="cursor-pointer text-sm text-white bg-opacity-50 bg-success uppercase"
            on:click=move |_| {
                sort.update(|i| *i = !*i);
                sort_by.set(header.get().sort_name);
            }
        >
            <div class="flex justify-between">
                <span class="flex-0">{move || header.get().display_name}</span>
                <span class="flex-0">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="w-5 h-5"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z"
                            clip-rule="evenodd"
                        ></path>
                    </svg>
                </span>
            </div>
        </th>
    }
}

#[allow(non_snake_case)]
#[component]
fn TableRow(sort_by: RwSignal<String>, header: RwSignal<TableHeader>, sort: RwSignal<bool>) -> impl IntoView {
    view! {
        <th
            class="cursor-pointer"
            on:click=move |_| {
                sort.update(|i| *i = !*i);
                sort_by.set(header.get().sort_name);
            }
        >
            {move || header.get().display_name}
        </th>
    }
}

/// Pagination component for the DirectusDataTable
/// 
/// # Arguments
/// 
/// * `total` - Total number of rows in the table
/// * `limit` - Number of rows to display per page
/// * `offset` - The current offset of the table (start with 0)
/// * `current_page` - The current page number
#[allow(non_snake_case)]
#[component]
pub fn TablePagination(
    total: RwSignal<u32>,
    limit: RwSignal<u32>,
    offset: RwSignal<u32>,
    current_page: RwSignal<u32>
) -> impl IntoView {
    let previous_disabled = move || current_page.get() == 1;
    let aggregated_button = move || {
        let mut buttons = vec![];
        let total_page = total.get() / limit.get();
        for i in 1..=total_page {
            buttons.push(i);
        }
        if total.get() % limit.get() != 0 {
            buttons.push(total_page + 1);
        }
        buttons
    };
    let row_from = move || offset.get() + 1;
    let row_to = move || {
        let r = row_from() + limit.get() - 1;
        if r > total.get() {
            total.get()
        } else {
            r
        }
    };

    let show_pagination = move || limit.get() < total.get();
    let next_disabled = move || current_page.get() == (aggregated_button().len() as u32);
    
    view! {
        <div class="flex justify-between w-full">
            <div class="flex-auto">
                <span>
                    {move || {
                        format!("Showing {} to {} of {} entries", row_from(), row_to(), total.get())
                    }}
                </span>
            </div>
            <Show when=move || show_pagination()>
                <div class="flex flex-auto justify-end">
                    <button
                        class="btn btn-ghost btn-xs"
                        prop:disabled=move || previous_disabled()
                        on:click=move |_| {
                            current_page.update(|i| *i = *i - 1);
                            offset.set((current_page.get() - 1) * limit.get());
                        }
                    >

                        Previous
                    </button>

                    {move || {
                        aggregated_button()
                            .into_iter()
                            .map(|i| {
                                view! {
                                    <button
                                        class="btn btn-square btn-xs"
                                        prop:disabled=move || current_page.get() == i
                                        on:click=move |_| {
                                            current_page.set(i);
                                            offset.set((current_page.get() - 1) * limit.get());
                                        }
                                    >
                                        {i}
                                    </button>
                                }
                            })
                            .collect_view()
                    }}

                    <button
                        class="btn btn-ghost btn-xs"
                        on:click=move |_| {
                            current_page.update(|i| *i = *i + 1);
                            offset.set((current_page.get() - 1) * limit.get());
                        }

                        prop:disabled=move || next_disabled()
                    >
                        Next
                    </button>
                </div>
            </Show>
        </div>
    }
}


#[allow(non_snake_case)]
#[component]
pub fn DownloadCsvAnchor(
    content: String,
    file_name: String,
    #[prop(optional)] button_name: String,
) -> impl IntoView {
    use wasm_bindgen::JsValue;
    use web_sys::{
        js_sys::{Array, Uint8Array},
        Blob, BlobPropertyBag,
    };
    let new_file_name = move || {
        let utc = chrono::Utc::now();
        let utc_local = utc.with_timezone(&chrono::Local);
        let formatted_local = utc_local.format("%Y%m%d_%H%M%S").to_string();
        format!("{}_{}.csv", formatted_local, file_name)
    };
    let button_placeholder = move || match button_name.len() > 0 {
        true => button_name,
        false => String::from("Download"),
    };
    let download = move || {
        let uint8arr = Uint8Array::new(&unsafe { Uint8Array::view(&content.as_bytes()) }.into());
        let array = Array::new();
        array.push(&uint8arr.buffer());
        let file = Blob::new_with_u8_array_sequence_and_options(
            &JsValue::from(array),
            BlobPropertyBag::new().type_("text/csv"),
        )
        .unwrap();
        let doc = leptos_dom::document();
        let hyperlink = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlAnchorElement>(
            doc.create_element("a").unwrap(),
        )
        .unwrap();
        hyperlink.set_download(new_file_name().as_str());
        let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
        hyperlink.set_href(&url);
        hyperlink.click();
        hyperlink.remove();
    };
    view! {
        <div>
            <button
                class="font-normal btn btn-sm btn-ghost bg-base-100 rounded-sm"
                on:click=move |_| download()
            >
                <div class="flex gap-2 justify-normal text-center items-center content-center">
                    <span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            class="w-4 h-4"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4.5 2A1.5 1.5 0 003 3.5v13A1.5 1.5 0 004.5 18h11a1.5 1.5 0 001.5-1.5V7.621a1.5 1.5 0 00-.44-1.06l-4.12-4.122A1.5 1.5 0 0011.378 2H4.5zm4.75 6.75a.75.75 0 011.5 0v2.546l.943-1.048a.75.75 0 011.114 1.004l-2.25 2.5a.75.75 0 01-1.114 0l-2.25-2.5a.75.75 0 111.114-1.004l.943 1.048V8.75z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                    </span>
                    <span class="font-extralight">{button_placeholder()}</span>
                </div>
            </button>
        </div>
    }
}

