'use client'

import Link from "next/link";
import useSWR from 'swr'

type Fetcher = (...args: Parameters<typeof fetch>) => Promise<any>;

const fetcher: Fetcher = async (url) => fetch(url, {
    method: 'GET',
    headers: {
        'Content-Type': 'application/json',
    },
    credentials: 'same-origin' as RequestCredentials, // Use 'include' and cast to RequestCredentials
}).then((res) => res.text())

export default function Profile() {
    const { data } = useSWR('/api/user/name', fetcher)
    return (
        <>
            <div className="absolute top-0 right-0 m-4 p-2 bg-gray-800 text-white rounded">
                {data || <Link href="/auth">Login</Link>}
            </div>
        </>
    )
}