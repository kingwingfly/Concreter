'use client'

import { ArticleInfo, useArticle, useArticles } from "@/utils/useArticle"
import { useEntities, useEntity } from "@/utils/useEntity"
import Link from "next/link"
import Entity from "./entity"

export function ArticleList() {
    const articles = useArticles()
    return (
        <>
            <div className="flex-col justify-items-center w-3/4">
                {articles?.map((article: ArticleInfo) => (
                    <ArticleCard key={article.id} article={article} />
                ))}
            </div>
        </>
    )
}


export function ArticleView({ id }: { id: string }) {
    const article = useArticle(id)
    const entities = useEntities(id)
    return (
        <>
            <div className="w-full mt-8 shadow-md rounded-md">
                <h1 className="text-3xl font-bold mb-4">{article?.title}</h1>
                <p className="">{article?.content}</p>
            </div>
            <div className="w-1/4 hover:w-1/2 transition-all">
                {entities?.map((id) => <Entity key={id} id={id} />)}
            </div>
        </>
    )
}

function ArticleCard({ article }: { article: ArticleInfo }) {
    return (
        <Link href={`/article/${article.id}`} className="block mt-3 w-full">
            <div className="mx-auto bg-white shadow-md rounded-lg overflow-hidden">
                <div className="p-4">
                    <h2 className="text-xl font-semibold text-gray-800">{article.title}</h2>
                    <p className="text-gray-600 mt-2">{article.fragment}</p>
                </div>
                <div className="bg-gray-100 p-1 place-self-end">
                    <span className="text-sm text-gray-500">{article.field}</span>
                </div>
            </div>
        </Link>
    )
}