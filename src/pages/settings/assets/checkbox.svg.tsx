import { SVGProps } from "react";
const SVGComponent = (props: SVGProps<SVGSVGElement>) => (
    <svg
        width={17}
        height={17}
        viewBox="0 0 17 17"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        {...props}
    >
        <path
            d="M2 4C2 2.89543 2.89543 2 4 2H15C16.1046 2 17 2.89543 17 4V15C17 16.1046 16.1046 17 15 17H4C2.89543 17 2 16.1046 2 15V4Z"
            fill="currentColor"
        />
        <path
            fillRule="evenodd"
            clipRule="evenodd"
            d="M14 1H2C1.44772 1 1 1.44772 1 2V5.5C1 5.77614 0.776142 6 0.5 6V6C0.223858 6 0 5.77614 0 5.5V2C0 0.895431 0.895431 0 2 0H14C15.1046 0 16 0.895431 16 2V4.5C16 4.77614 15.7761 5 15.5 5V5C15.2239 5 15 4.77614 15 4.5V2C15 1.44772 14.5523 1 14 1ZM0.5 11C0.223858 11 0 11.2239 0 11.5V14C0 15.1046 0.895431 16 2 16H14C15.1046 16 16 15.1046 16 14V8.5C16 8.22386 15.7761 8 15.5 8V8C15.2239 8 15 8.22386 15 8.5V14C15 14.5523 14.5523 15 14 15H2C1.44772 15 1 14.5523 1 14V11.5C1 11.2239 0.776142 11 0.5 11V11Z"
            fill="#3A3A3A"
        />
    </svg>
);
export default SVGComponent;
