import constate from "constate";
import { useCallback } from "react";
import { RouteKey, routerService } from "../core/router.service";
import { useForceUpdate } from "../utils";


function useRouter_() {
    const forceUpdate = useForceUpdate()

    const navigate = useCallback((key: RouteKey) => {
        routerService.setKey(key)
        forceUpdate()
    }, [])

    return {
        routeKey: routerService.getKey(),
        navigate,
    }
}

export const [RouterProvider, useRouteKey, useNavigate] = constate(
    useRouter_,
    v => v.routeKey,
    v => v.navigate
)
