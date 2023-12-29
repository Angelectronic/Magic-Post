import fakeRestProvider from './rest';
import { DataProvider, withLifecycleCallbacks, HttpError } from 'react-admin';
import data from './data';

const dataProvider = withLifecycleCallbacks(fakeRestProvider(data, true), [
    {
        resource: 'posts',
    },
]);


const delayedDataProvider = new Proxy(dataProvider, {
  get: (target, name, self) =>
      name === 'then' // as we await for the dataProvider, JS calls then on it. We must trap that call or else the dataProvider will be called with the then method
          ? self
          : (resource: string, params: any) =>
                new Promise(resolve =>
                    setTimeout(
                        () =>
                            resolve(
                              dataProvider[name as string](resource, params)
                            ),
                        500
                    )
                ),
});

export default delayedDataProvider;