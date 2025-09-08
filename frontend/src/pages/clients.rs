use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use ghosthub_shared::Client;

#[function_component(ClientsPage)]
pub fn clients_page() -> Html {
    let clients = use_state(|| Vec::<Client>::new());
    let loading = use_state(|| true);

    {
        let clients = clients.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match Request::get("/api/v1/clients")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Client>>()
                    .await
                {
                    Ok(data) => {
                        clients.set(data);
                        loading.set(false);
                    }
                    Err(_) => {
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div>
            <div class="sm:flex sm:items-center">
                <div class="sm:flex-auto">
                    <h1 class="text-3xl font-bold text-gray-900">{"Clients"}</h1>
                    <p class="mt-2 text-sm text-gray-700">{"Manage your MSP clients and their information"}</p>
                </div>
                <div class="mt-4 sm:ml-16 sm:mt-0 sm:flex-none">
                    <button type="button" class="block rounded-md bg-blue-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600">
                        {"Add Client"}
                    </button>
                </div>
            </div>

            <div class="mt-8 flow-root">
                <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                        if *loading {
                            <div class="bg-white shadow rounded-lg p-6">
                                <p class="text-gray-500">{"Loading clients..."}</p>
                            </div>
                        } else if clients.is_empty() {
                            <div class="bg-white shadow rounded-lg p-6">
                                <p class="text-gray-500">{"No clients found. Create your first client to get started."}</p>
                            </div>
                        } else {
                            <table class="min-w-full divide-y divide-gray-300">
                                <thead class="bg-gray-50">
                                    <tr>
                                        <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">{"Name"}</th>
                                        <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">{"Email"}</th>
                                        <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">{"Phone"}</th>
                                        <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">{"City"}</th>
                                        <th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
                                            <span class="sr-only">{"Actions"}</span>
                                        </th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-gray-200 bg-white">
                                    {
                                        clients.iter().map(|client| {
                                            html! {
                                                <tr key={client.id.to_string()}>
                                                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">{&client.name}</td>
                                                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{client.email.as_deref().unwrap_or("-")}</td>
                                                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{client.phone.as_deref().unwrap_or("-")}</td>
                                                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{client.city.as_deref().unwrap_or("-")}</td>
                                                    <td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                                                        <a href="#" class="text-blue-600 hover:text-blue-900">{"Edit"}</a>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Html>()
                                    }
                                </tbody>
                            </table>
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}