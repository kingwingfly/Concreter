import { ButtonLink } from "@/components/btn";
import PostForm from "@/components/form";

export default function Auth() {
  return (
    <>
      <div className="grid grid-col-1 justify-items-center">
        <PostForm url="/api/login" keys={["username", "password"]} btn="Login" />
        <ButtonLink href="/auth/register" name="Register" />
      </div>
    </>
  )
}
