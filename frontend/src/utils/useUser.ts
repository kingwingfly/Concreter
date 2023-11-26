import useSWR from "swr"
import { fetcher } from "./fetcher"

export interface User {
    id: number
    username: string
}

export const useUser = () => {
    const { data } = useSWR('/api/user', fetcher)
    let user: User = data
    return { ...user }
}