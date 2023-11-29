import { ArticleView } from "@/components/article"

interface Params { id: string }

export default function Article({ params }: { params: Params }) {
    return (
        <div className="h-screen w-screen flex">
            <ArticleView id={params.id} />
        </div>
    )
}


export async function generateStaticParams() {
    let resp = await fetch("http://localhost:8080/api/article/ids", {
        method: "GET",
        headers: {
            'Content-Type': 'application/json',
        },
        cache: 'no-store'
    })
    let ret: number[] = await resp.json()

    return ret.map((id) => {
        return { id: id.toString() }
    })
}