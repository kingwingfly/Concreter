import Link from "next/link"

interface ButtonProps extends React.HTMLAttributes<HTMLButtonElement> { name: string }

export default function SubmitBtn(props: ButtonProps) {
  return (
    <button {...props} type="submit"
      className="rounded-md bg-indigo-500 mt-4 w-fit h-fit px-3.5 py-2 text-sm font-semibold text-white shadow-sm 
      hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
    > {props.name} </button>
  )
}

export function ButtonLink(props: { name: string, href: string }) {
  return (
    <>
      <div className="rounded-md bg-indigo-500 mt-4 w-fit h-fit px-3.5 py-2 text-sm font-semibold text-white shadow-sm
        hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
      >
        <Link href={props.href}> {props.name} </Link>
      </div>
    </>
  )
}