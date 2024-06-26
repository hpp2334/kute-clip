import { SVGProps } from "react"
const SvgComponent = (props: SVGProps<SVGSVGElement>) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={82}
    height={82}
    fill="none"
    {...props}
  >
    <rect width={80} height={80} x={2} y={2} fill="#FFDA7B" rx={40} />
    <path
      fill="#F6C667"
      fillRule="evenodd"
      d="M8.849 64.39C16.039 75.016 28.204 82 42 82c22.091 0 40-17.909 40-40 0-10.598-4.121-20.233-10.849-27.39A39.815 39.815 0 0 1 78.001 37c0 22.091-17.91 40-40 40-11.495 0-21.856-4.848-29.152-12.61Z"
      clipRule="evenodd"
    />
    <path
      fill="#3A3A3A"
      fillRule="evenodd"
      d="M40 79C18.46 79 1 61.54 1 40S18.46 1 40 1c15.453 0 28.807 8.988 35.12 22.021a.506.506 0 0 0 .585.27c.301-.08.457-.414.32-.694C69.554 9.223 55.855 0 40 0 17.909 0 0 17.909 0 40s17.909 40 40 40 40-17.909 40-40c0-4.22-.653-8.288-1.865-12.107a.495.495 0 0 0-.601-.326.505.505 0 0 0-.349.64A38.98 38.98 0 0 1 79 40c0 21.54-17.46 39-39 39Z"
      clipRule="evenodd"
    />
    <path
      fill="#fff"
      fillRule="evenodd"
      d="M29.248 8.777a.495.495 0 0 0-.656-.257c-.638.287-1.268.59-1.89.91a.495.495 0 0 0-.21.674c.13.244.433.337.678.21.6-.308 1.206-.6 1.82-.877a.505.505 0 0 0 .258-.66ZM11.812 23.58c.232.15.541.083.694-.147a39 39 0 0 1 10.96-10.95.505.505 0 0 0 .149-.693.495.495 0 0 0-.69-.147 40 40 0 0 0-11.26 11.248.495.495 0 0 0 .147.69Z"
      clipRule="evenodd"
    />
    <path
      stroke="#3A3A3A"
      strokeLinecap="round"
      strokeWidth={1.5}
      d="M21.75 34.25h8.5M49 34h10"
    />
    <path
      fill="#3A3A3A"
      d="M36.042 47.333h1.513c-.365.621-.644 1.174-.835 1.66a6.69 6.69 0 0 0-.384 1.376c-.068.432-.102.88-.102 1.343 0 1.015.16 1.803.48 2.364.324.56.766.84 1.326.84.493 0 .886-.217 1.18-.654.296-.44.445-1.08.445-1.918v-2.596h1.06v2.596c0 .816-.108 1.508-.327 2.076-.214.568-.53 1-.948 1.298-.417.293-.925.44-1.523.44-.65 0-1.207-.18-1.67-.542-.459-.36-.81-.88-1.055-1.557-.245-.68-.367-1.5-.367-2.46 0-.782.1-1.527.3-2.234.199-.708.501-1.385.907-2.032Zm7.29 0h1.513c.406.647.709 1.324.908 2.032.2.707.3 1.452.3 2.234 0 .96-.123 1.78-.368 2.46-.244.677-.598 1.196-1.06 1.557-.46.361-1.014.542-1.665.542-.598 0-1.106-.147-1.523-.44-.418-.297-.736-.73-.954-1.298-.214-.568-.322-1.26-.322-2.076v-2.596h1.061v2.596c0 .839.149 1.478.446 1.918.297.437.69.655 1.18.655.375 0 .697-.126.964-.378.27-.252.478-.617.62-1.095.147-.478.22-1.055.22-1.732 0-.463-.033-.91-.1-1.343a6.499 6.499 0 0 0-.39-1.377 11.702 11.702 0 0 0-.83-1.659Z"
    />
    <rect width={4} height={4} x={21} y={45} fill="#EFA874" rx={2} />
    <rect width={4} height={4} x={56} y={45} fill="#EFA874" rx={2} />
  </svg>
)
export default SvgComponent
