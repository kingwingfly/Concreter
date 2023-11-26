'use client'

import { FormEvent } from "react"
import Input from "./input"
import { useRouter } from "next/navigation"
import { useState } from "react"
import SubmitBtn from "./btn"


export default function PostForm(props: { url: string, keys: string[], btn: string }) {
    const router = useRouter()
    let [ret, setRet] = useState("")
    async function submit(e: FormEvent<HTMLFormElement>) {
        let form = new FormData(e.currentTarget)
        let json = Object.fromEntries(form)
        let resp = await fetch(props.url, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(json)
        })
        if (resp.status == 200) { router.back() } else { setRet("Invalid username or password") }
    }

    return (
        <>
            <form className="flex flex-col place-items-center overflow-y-auto" onSubmit={(e) => { e.preventDefault(); submit(e); }}>
                {props.keys.map((key) => {
                    return (
                        <div key={key} className="grid grid-cols-3 mt-2 place-content-center">
                            <label className="col-start-1 col-span-1 place-items-center mx-2 py-2"> {key} </label>
                            <div className="col-start-2 col-span-2">
                                <Input name={key} required />
                            </div>
                        </div>
                    )
                })}
                <SubmitBtn name={props.btn} />
                <div>{ret}</div>
            </form>
        </>
    )
}