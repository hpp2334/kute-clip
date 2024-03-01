import { SVGProps } from "react";
const SVGComponent = (props: SVGProps<SVGSVGElement>) => (
  <svg
    width={12}
    height={12}
    viewBox="0 0 12 12"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    {...props}
  >
    <path
      d="M9 2L2 6L9 10"
      stroke="#3A3A3A"
      strokeLinecap="round"
      strokeLinejoin="round"
    />
  </svg>
);
export default SVGComponent;
