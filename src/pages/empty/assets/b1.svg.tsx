import { SVGProps } from "react"
const SvgComponent = (props: SVGProps<SVGSVGElement>) => (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        width={120}
        height={40}
        fill="none"
        {...props}
    >
        <path
            fill="#DEEBFF"
            fillRule="evenodd"
            d="M0 4a4 4 0 0 1 4-4h12a4 4 0 0 1 0 8H4a4 4 0 0 1-4-4Zm30 0a4 4 0 0 1 4-4h72a4 4 0 0 1 0 8H90a4 4 0 0 0 0 8h26a4 4 0 0 1 0 8h-15a4 4 0 0 0 0 8h10a4 4 0 0 1 0 8H24a4 4 0 0 1 0-8 4 4 0 0 0 0-8h-8a4 4 0 0 1 0-8h45a4 4 0 0 0 0-8H34a4 4 0 0 1-4-4ZM13 36a4 4 0 0 0-8 0 4 4 0 0 0 8 0Zm45-8a4 4 0 0 1 4-4h21a4 4 0 0 1 0 8H62a4 4 0 0 1-4-4Z"
            clipRule="evenodd"
        />
    </svg>
)
export default SvgComponent
