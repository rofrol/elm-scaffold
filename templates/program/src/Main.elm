module Main exposing (main)

import Html


type alias Model =
    Int


type Msg
    = NoOp


main : Program Never Model Msg
main =
    Html.program
        { view = \_ -> Html.text "Hello, World!"
        , update = \_ _ -> ( 1, Cmd.none )
        , subscriptions = \_ -> Sub.none
        , init = ( 1, Cmd.none )
        }
