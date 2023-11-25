import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Concreter",
  description: "A note enhancement tool with openai and sympy",
};

export default function RootLayout(props: {
  children: React.ReactNode,
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="flex h-screen w-auto items-center justify-center">
          {props.children}
        </div>
      </body>
    </html>
  )
}
