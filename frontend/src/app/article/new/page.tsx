import PostForm from "@/components/form";

export default function NewArticle() {
    return (
        <>
            <div className="grid grid-col-1 justify-items-center">
                <PostForm url="/api/article" keys={["title", "content", "field"]} btn="Create" />
            </div>
        </>
    )
}