import PostForm from "@/components/form";

export default function Auth() {
  return (
    <>
      <PostForm url="/api/login" keys={["username", "password"]} btn="login" />
    </>
  )
}
