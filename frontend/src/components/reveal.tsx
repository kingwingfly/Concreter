'use client'

import { useEffect } from 'react'
import Reveal from 'reveal.js'
import RevealMarkdown from 'reveal.js/plugin/markdown/markdown'
import RevealMath from 'reveal.js/plugin/math/math'
import RevealHighlight from 'reveal.js/plugin/highlight/highlight'

import 'reveal.js/dist/reveal.css'
import 'reveal.js/dist/theme/black.css'


export const ArticleContent = ({ id }: { id: string }) => {
    useEffect(
        () => {
            let deck = new Reveal({
                plugins: [RevealMarkdown, RevealMath.KaTeX, RevealHighlight]
            })
            deck.initialize({
                embedded: true,
            })
        }, []
    )
    return (
        <div className="reveal">
            <div className="slides">
                <section data-markdown={`/api/md/${id}`} data-separator="^---$" data-separator-vertical="^--$">
                </section>
            </div>
        </div>
    )
}
