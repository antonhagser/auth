import styles from "./page.module.css";

export default async function Home() {
    return (
        <main className={styles.main}>
            <header className={styles.header}>
                <div className={styles.headerName}>
                    <h1>Auth</h1>
                </div>
                <div className={styles.headerLinks}>
                    <nav>
                        <li>
                            <a href="/login">Login</a>
                        </li>
                        <li>
                            <a href="/register">Register</a>
                        </li>
                    </nav>
                </div>
            </header>
        </main>
    );
}
