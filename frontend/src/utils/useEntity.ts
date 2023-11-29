import useSWR from "swr"
import { fetcher } from "./fetcher"

export interface KV {
    [key: string]: string | KV
}

export interface Entity {
    id: number,
    name: string,
    attris: KV,
}

export const useEntity = (id: string) => {
    const { data, isLoading } = useSWR(`/api/entity/${id}`, fetcher)
    if (isLoading) return { id: 0, name: '', attris: {} }
    let entity = data as Entity

    return entity
}

export const useEntities = (id: string) => {
    const { data } = useSWR(`/api/article/${id}/entities`, fetcher)
    let articles: string[] = data

    return articles
}