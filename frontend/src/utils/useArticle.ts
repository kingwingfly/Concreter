import useSWR from "swr"
import { fetcher } from "./fetcher"

export interface Article {
    id: number
    title: string,
    content: string,
    entities?: number[],
    formulas?: number[],
}

export const useArticle = (id: string) => {
    const { data } = useSWR(`/api/article/${id}`, fetcher)
    let article: Article = data

    return article
}

export interface ArticleInfo {
    id: number
    title: string,
    fragment: string,
    field: string,
}

export const useArticles = () => {
    const { data } = useSWR('/api/articles', fetcher)
    let articles: ArticleInfo[] = data

    return articles
}