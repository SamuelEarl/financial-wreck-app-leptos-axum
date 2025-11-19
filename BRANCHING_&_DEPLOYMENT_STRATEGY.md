# Branching Strategy

These are the branches that will exist in this repo and their purposes:

## main

* Used for production deployments.
* Builds must always be in a stable, working state and able to deploy (i.e. `main` should never contain broken code).
* All production deployments should run through a CI/CD pipeline that runs automated tests before deployment. If all tests pass, then deployment continues.
* After a production deployment is live, all manual and automated tests for the live production app should occur.

## staging

* The `staging` branch and environment should be an exact clone of the `main` branch and environment.
* Used as the integration branch for all feature/development branches (i.e. all feature/development branches will branch off of `staging`).
* Staging deployments occur from `staging`.
* Only after all the manual/automated tests pass should you merge the `staging` branch into `main` and test a production deployment.
* All `staging` deployments should run through a CI/CD pipeline that runs automated tests before deployment. If all tests pass, then deployment continues.
* After a `staging` deployment is live, all manual and automated tests for the live `statging` app should occur.


## feature/development branches

* All feature/development branches will branch off of `staging`.
* When a feature branch is complete you should test a preview deployment and then a `staging` deployment.
    * The `staging` branch should be merged into the feature branch and the feature branch should be tested within it's own branch/environment.
    * Only after all the manual/automated tests pass should you merge the feature branch into `staging` and test a `staging` deployment.
