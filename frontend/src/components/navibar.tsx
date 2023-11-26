'use client'

import { useUser } from "@/utils/useUser";
import { ButtonLink } from "./btn";


export default function NaviBar() {
    const { username } = useUser()
    return (
        <>
            <div className="absolute top-0 right-0 m-4 p-2 rounded flex">
                {username && <ButtonLink href="/article/new" name="New Article" />}
                {username ? <div className="px-3.5 py-2 text-sm font-semibold mt-4 mx-1 rounded bg-indigo-500">{username}</div> : <ButtonLink href="/auth" name="Login" />}
            </div>
        </>
    )
}