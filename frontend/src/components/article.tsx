'use client'

import { Article, ArticleInfo, useArticles } from "@/utils/useArticle"
import Link from "next/link"

export function ArticleList() {
    const articles = useArticles()
    return (
        <>
            <div className="grid grid-cols-1 justify-items-center">
                {articles?.map((article: ArticleInfo) => (
                    <ArticleCard key={article.id} article={article} />
                ))}
            </div>
        </>
    )
}


export function Article({ article }: { article: Article }) {
    return (
        <>
            <div className="grid grid-cols-1 justify-items-center">
                <h1>{article.title}</h1>
                <div>{article.content}</div>
            </div>
        </>
    )
}

function ArticleCard({ article }: { article: ArticleInfo }) {
    return (
        <>
            <Link href={`/article/${article.id}`} />
        </>
    )
}