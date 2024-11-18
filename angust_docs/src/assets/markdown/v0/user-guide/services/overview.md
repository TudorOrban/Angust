
&nbsp;

# Services

A **Service** is any piece of code encapsulating some focused business logic. For example, you may want a service that is responsible for:

- fetching data from the backend or
- executing a resource-intensive algorithm or
- validating some form data

In fact, Angust strongly recommends designing your application so that any business logic that is not related to the UI is neatly encapsulated in a service. This solid pattern will allow you to develop *scalable* and *decoupled* codebases.

To facilitate this design, Angust provides a *Service Registry*, i.e. a global registry from which you can access your services anywhere in the application. It also provides support for executing async operations and safely handling their responses.

&nbsp;

## Next Step

Learn how you can [register and use](https://tudororban.github.io/Angust/v0/user-guide/services/registration-and-usage) services.

&nbsp;