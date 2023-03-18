/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export const $MoveFunction = {
    description: `Move function`,
    properties: {
        name: {
            type: 'IdentifierWrapper',
            isRequired: true,
        },
        visibility: {
            type: 'MoveFunctionVisibility',
            isRequired: true,
        },
        is_entry: {
            type: 'boolean',
            description: `Whether the function can be called as an entry function directly in a transaction`,
            isRequired: true,
        },
        is_view_function: {
            type: 'boolean',
            description: `Whether the function is a view function. If we didn't look this up (since this
            requires us to invoke the VM), this will be None.`,
        },
        generic_type_params: {
            type: 'array',
            contains: {
                type: 'MoveFunctionGenericTypeParam',
            },
            isRequired: true,
        },
        params: {
            type: 'array',
            contains: {
                type: 'MoveType',
            },
            isRequired: true,
        },
        return: {
            type: 'array',
            contains: {
                type: 'MoveType',
            },
            isRequired: true,
        },
    },
} as const;
