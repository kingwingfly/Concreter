'use client'

import { ChangeEvent, FormEvent } from "react"
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
        if (resp.status == 200) { router.back() } else { setRet("Invalid Input") }
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


export const FileUploadComponent = () => {
    const [selectedFile, setSelectedFile] = useState<File | null>(null)
    const [field, setField] = useState<string>("")
    const [uploading, setUploading] = useState<boolean>(false)
    const router = useRouter()

    const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        setSelectedFile(file || null);
    };

    const handleFiledChange = (e: ChangeEvent<HTMLInputElement>) => {
        const field = e.target.value;
        setField(field);
    }

    const handleUpload = async () => {
        if (selectedFile && field) {
            setUploading(true);
            const formData = new FormData();
            formData.append('filename', selectedFile.name)
            formData.append('content', selectedFile);
            formData.append('field', field);
            const response = await fetch('/api/article', {
                method: 'POST',
                body: formData,
            });
            if (response.ok) {
                router.back()
            } else {
                alert('上传失败')
                setUploading(false)
            }
        } else {
            alert('请选择文件和领域');
        }
    };

    return (
        <div className="p-4 w-fit grid grid-cols-4">
            <input
                type="file"
                className="col-span-2 mx-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                onChange={(e) => { e.preventDefault(); handleFileChange(e) }}
            />
            <input
                type="text"
                value={field}
                placeholder="请输入领域"
                className="col-span-1 mx-4 bg-white text-black dark:bg-black dark:text-white
                ring-2 ring-blue-400 hover:ring-2 hover:ring-blue-600 font-bold py-2 px-4 rounded"
                onChange={(e) => { e.preventDefault(); handleFiledChange(e) }}
            />
            <button
                className="mx-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                onClick={(e) => { e.preventDefault(); handleUpload() }}
                disabled={uploading}
            >
                {uploading ? '上传中' : '上传文件'}
            </button>
        </div>
    );
};
