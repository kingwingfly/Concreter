import PostForm from "@/components/form";

export default function Auth() {
    return (
        <>
            <PostForm url="/api/register" keys={["username", "password"]} btn="Register" />
        </>
    )
}
