

export const enum RouteKey {
    Main,
    Setting,
}

export class RouterService {
    private _routeKey = RouteKey.Main;

    public getKey() {
        return this._routeKey
    }

    public setKey(key: RouteKey) {
        this._routeKey = key
    }
}

export const routerService = new RouterService()
