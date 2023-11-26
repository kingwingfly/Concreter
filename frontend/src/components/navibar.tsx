'use client'

import { useUser } from "@/utils/useUser";
import Link from "next/link";


export default function NaviBar() {
    const { username } = useUser()
    return (
        <>
            <div className="absolute top-0 right-0 m-4 p-2 bg-gray-800 text-white rounded">
                {username || <Link href="/auth">Login</Link>}
            </div>
        </>
    )
}