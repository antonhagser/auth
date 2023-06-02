import styles from "./styles.css";

export const links = () => [{ rel: "stylesheet", href: styles }];

const sizeMap = {
    tiny: [6, 5],
    small: [8, 7],
    medium: [12, 10.5],
    large: [16, 14],
};

interface Props {
    className?: string;
    size: "tiny" | "small" | "medium" | "large";
}

export function Spinner({ className, size }: Props) {
    return (
        <svg
            className={combineClassNames("spinner", className)}
            data-size={size}
        >
            <circle
                cx={sizeMap[size][0]}
                cy={sizeMap[size][0]}
                r={sizeMap[size][1]}
            ></circle>
        </svg>
    );
}

function combineClassNames(...classNames: (string | undefined)[]) {
    return classNames.filter(Boolean).join(" ");
}
