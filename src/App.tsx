import { EmptyPage } from "./pages/empty";
import styles from './app.module.scss'
import { useView } from "./core/view.service";
import { MainPage } from "./pages/main";
import { RouterProvider, useRouteKey } from "./widgets/Router";
import { SettingPage } from "./pages/settings";
import { RouteKey } from "./core/router.service";


function Routes() {
  const routeKey = useRouteKey()
  const historyList = useView('clipboard_history');

  return (
    <>
      {routeKey === RouteKey.Main && (
        <>
          {!historyList?.items.length && <EmptyPage />}
          {Boolean(historyList?.items.length) && <MainPage />}
        </>
      )}
      {routeKey == RouteKey.Setting && (
        <SettingPage />
      )}
    </>
  )
}

function App() {
  return (
    <RouterProvider>
      <div className={styles.container}>
        <Routes />
      </div>
    </RouterProvider>
  );
}

export default App;
