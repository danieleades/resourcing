/* eslint-disable */
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = {
	[_ in K]?: never;
};
export type Incremental<T> =
	| T
	| { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
	ID: { input: string; output: string };
	String: { input: string; output: string };
	Boolean: { input: boolean; output: boolean };
	Int: { input: number; output: number };
	Float: { input: number; output: number };
	Month: { input: any; output: any };
	/**
	 * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
	 * Strings within GraphQL. UUIDs are used to assign unique identifiers to
	 * entities without requiring a central allocating authority.
	 *
	 * # References
	 *
	 * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
	 * * [RFC4122: A Universally Unique Identifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
	 */
	UUID: { input: any; output: any };
};

export type AssignInput = {
	month: Scalars['Month']['input'];
	projectId: Scalars['UUID']['input'];
	resourceId: Scalars['UUID']['input'];
};

export type Assignment = {
	__typename?: 'Assignment';
	id: Scalars['String']['output'];
	month: Scalars['Month']['output'];
	project: Project;
	resource: Resource;
};

export type CreateProjectInput = {
	name: Scalars['String']['input'];
};

export type CreateResourceInput = {
	name: Scalars['String']['input'];
};

export type MutationRoot = {
	__typename?: 'MutationRoot';
	assign: Assignment;
	createProject: Project;
	createResource: Resource;
	deleteProject: Scalars['Boolean']['output'];
	deleteResource: Scalars['Boolean']['output'];
	unassign: Scalars['Boolean']['output'];
};

export type MutationRootAssignArgs = {
	input: AssignInput;
};

export type MutationRootCreateProjectArgs = {
	input: CreateProjectInput;
};

export type MutationRootCreateResourceArgs = {
	input: CreateResourceInput;
};

export type MutationRootDeleteProjectArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRootDeleteResourceArgs = {
	id: Scalars['UUID']['input'];
};

export type MutationRootUnassignArgs = {
	input: UnassignInput;
};

export type Project = {
	__typename?: 'Project';
	assignments: Array<Assignment>;
	id: Scalars['UUID']['output'];
	name: Scalars['String']['output'];
};

export type ProjectMonthCell = {
	__typename?: 'ProjectMonthCell';
	resources: Array<Resource>;
};

export type ProjectMonthMatrix = {
	__typename?: 'ProjectMonthMatrix';
	months: Array<Scalars['Month']['output']>;
	rows: Array<ProjectMonthMatrixRow>;
};

export type ProjectMonthMatrixRow = {
	__typename?: 'ProjectMonthMatrixRow';
	cells: Array<ProjectMonthCell>;
	project: Project;
};

export type QueryRoot = {
	__typename?: 'QueryRoot';
	assignments: Array<Assignment>;
	project?: Maybe<Project>;
	projectMonthMatrix: ProjectMonthMatrix;
	projects: Array<Project>;
	resource?: Maybe<Resource>;
	resourceMonthMatrix: ResourceMonthMatrix;
	resources: Array<Resource>;
};

export type QueryRootProjectArgs = {
	id: Scalars['UUID']['input'];
};

export type QueryRootProjectMonthMatrixArgs = {
	months: Array<Scalars['Month']['input']>;
};

export type QueryRootResourceArgs = {
	id: Scalars['UUID']['input'];
};

export type QueryRootResourceMonthMatrixArgs = {
	months: Array<Scalars['Month']['input']>;
};

export type Resource = {
	__typename?: 'Resource';
	assignments: Array<Assignment>;
	id: Scalars['UUID']['output'];
	name: Scalars['String']['output'];
};

export type ResourceMonthCell = {
	__typename?: 'ResourceMonthCell';
	projects: Array<Project>;
};

export type ResourceMonthMatrix = {
	__typename?: 'ResourceMonthMatrix';
	months: Array<Scalars['Month']['output']>;
	rows: Array<ResourceMonthMatrixRow>;
};

export type ResourceMonthMatrixRow = {
	__typename?: 'ResourceMonthMatrixRow';
	cells: Array<ResourceMonthCell>;
	resource: Resource;
};

export type UnassignInput = {
	month: Scalars['Month']['input'];
	projectId: Scalars['UUID']['input'];
	resourceId: Scalars['UUID']['input'];
};

export type GetProjectMonthMatrixQueryVariables = Exact<{
	months: Array<Scalars['Month']['input']> | Scalars['Month']['input'];
}>;

export type GetProjectMonthMatrixQuery = {
	__typename?: 'QueryRoot';
	projectMonthMatrix: {
		__typename?: 'ProjectMonthMatrix';
		months: Array<any>;
		rows: Array<{
			__typename?: 'ProjectMonthMatrixRow';
			project: { __typename?: 'Project'; id: any; name: string };
			cells: Array<{
				__typename?: 'ProjectMonthCell';
				resources: Array<{ __typename?: 'Resource'; id: any; name: string }>;
			}>;
		}>;
	};
};

export const GetProjectMonthMatrixDocument = {
	kind: 'Document',
	definitions: [
		{
			kind: 'OperationDefinition',
			operation: 'query',
			name: { kind: 'Name', value: 'GetProjectMonthMatrix' },
			variableDefinitions: [
				{
					kind: 'VariableDefinition',
					variable: { kind: 'Variable', name: { kind: 'Name', value: 'months' } },
					type: {
						kind: 'NonNullType',
						type: {
							kind: 'ListType',
							type: {
								kind: 'NonNullType',
								type: { kind: 'NamedType', name: { kind: 'Name', value: 'Month' } }
							}
						}
					}
				}
			],
			selectionSet: {
				kind: 'SelectionSet',
				selections: [
					{
						kind: 'Field',
						name: { kind: 'Name', value: 'projectMonthMatrix' },
						arguments: [
							{
								kind: 'Argument',
								name: { kind: 'Name', value: 'months' },
								value: { kind: 'Variable', name: { kind: 'Name', value: 'months' } }
							}
						],
						selectionSet: {
							kind: 'SelectionSet',
							selections: [
								{ kind: 'Field', name: { kind: 'Name', value: 'months' } },
								{
									kind: 'Field',
									name: { kind: 'Name', value: 'rows' },
									selectionSet: {
										kind: 'SelectionSet',
										selections: [
											{
												kind: 'Field',
												name: { kind: 'Name', value: 'project' },
												selectionSet: {
													kind: 'SelectionSet',
													selections: [
														{ kind: 'Field', name: { kind: 'Name', value: 'id' } },
														{ kind: 'Field', name: { kind: 'Name', value: 'name' } }
													]
												}
											},
											{
												kind: 'Field',
												name: { kind: 'Name', value: 'cells' },
												selectionSet: {
													kind: 'SelectionSet',
													selections: [
														{
															kind: 'Field',
															name: { kind: 'Name', value: 'resources' },
															selectionSet: {
																kind: 'SelectionSet',
																selections: [
																	{ kind: 'Field', name: { kind: 'Name', value: 'id' } },
																	{ kind: 'Field', name: { kind: 'Name', value: 'name' } }
																]
															}
														}
													]
												}
											}
										]
									}
								}
							]
						}
					}
				]
			}
		}
	]
} as unknown as DocumentNode<GetProjectMonthMatrixQuery, GetProjectMonthMatrixQueryVariables>;
