type Fetcher = (...args: Parameters<typeof fetch>) => Promise<any>;

export const fetcher: Fetcher = async (url) => fetch(url, {
    method: 'GET',
    headers: {
        'Content-Type': 'application/json',
    },
    credentials: 'same-origin' as RequestCredentials, // Use 'include' and cast to RequestCredentials
}).then((res) => res.json())